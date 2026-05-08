use std::time::Duration;
use tokio::spawn;
use tokio::time::timeout;

use crate::config::Config;
use crate::error::PrinterError;
use anyhow::Result;
use btleplug::api::{
    BDAddr, Central, CentralEvent, CharPropFlags, Characteristic, Manager as _, Peripheral as _,
    ScanFilter, WriteType,
};
use btleplug::platform::{Adapter, Manager, Peripheral};
use futures::stream::StreamExt;

pub struct D30 {
    device: Peripheral,
    characteristic: Characteristic,
}

impl D30 {
    pub async fn new(config: &Config) -> Result<Self> {
        let device = D30::find_device(config).await?;

        device
            .connect()
            .await
            .map_err(|e| PrinterError::ConnectionFailed(e.to_string()))?;

        let characteristics = device.characteristics();
        let characteristic = characteristics
            .into_iter()
            .find(|chr| {
                chr.properties.contains(CharPropFlags::WRITE)
                    || chr.properties.contains(CharPropFlags::WRITE_WITHOUT_RESPONSE)
            })
            .ok_or(PrinterError::CharacteristicNotFound)?;

        Ok(Self {
            device,
            characteristic,
        })
    }

    pub async fn write(&self, data: &[u8]) -> Result<()> {
        self.device
            .write(&self.characteristic, data, WriteType::WithResponse)
            .await
            .map_err(|e| PrinterError::CommunicationError(e.to_string()).into())
    }

    async fn d30_filter(p: &Peripheral, addr: &Option<BDAddr>) -> bool {
        let properties = match p.properties().await {
            Ok(Some(prop)) => prop,
            _ => return false,
        };

        let local_name = properties.local_name.unwrap_or_default();
        debug!("Found BLE device: {}, {:?}", local_name, properties.address);

        if let Some(d30_addr) = addr {
            properties.address == *d30_addr
        } else {
            local_name == "D30"
        }
    }

    async fn scan(central: Adapter, addr: Option<BDAddr>) -> Result<Peripheral> {
        let mut events = central
            .events()
            .await
            .map_err(|e| PrinterError::AdapterError(e.to_string()))?;

        info!("Scanning Bluetooth devices");
        central
            .start_scan(ScanFilter::default())
            .await
            .map_err(|e| PrinterError::AdapterError(e.to_string()))?;

        while let Some(event) = events.next().await {
            if let CentralEvent::DeviceDiscovered(id) = event {
                if let Ok(peripheral) = central.peripheral(&id).await {
                    if D30::d30_filter(&peripheral, &addr).await {
                        let _ = central.stop_scan().await;
                        return Ok(peripheral);
                    }
                }
            }
        }

        let _ = central.stop_scan().await;
        Err(PrinterError::DeviceNotFound.into())
    }

    async fn find_device(config: &Config) -> Result<Peripheral> {
        let manager = Manager::new()
            .await
            .map_err(|e| PrinterError::AdapterError(e.to_string()))?;

        let adapters = manager
            .adapters()
            .await
            .map_err(|e| PrinterError::AdapterError(e.to_string()))?;

        let adapter = adapters
            .into_iter()
            .next()
            .ok_or(PrinterError::NoAdapterFound)?;

        let addr = config.get_addr().ok().flatten(); // Handle potential addr parse error gracefully
        let handle = spawn(async move { D30::scan(adapter, addr).await });
        let time_limit = Duration::from_secs(config.scan_time.unwrap_or(5));

        match timeout(time_limit, handle).await {
            Ok(Ok(result)) => result,
            Ok(Err(join_error)) => Err(PrinterError::Other(format!("Task panicked: {:?}", join_error)).into()),
            Err(_) => Err(PrinterError::DiscoveryTimeout.into()),
        }
    }
}
