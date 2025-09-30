use std::time::Duration;
use tokio::spawn;
use tokio::time::timeout;

use crate::config::Config;
use anyhow::{Context, Result, anyhow};
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

        if let Err(e) = device.connect().await.context("Connect to D30 Device.") {
            return Err(anyhow!("Failed to connect to D30: {}", e));
        }

        let characterics: Option<Characteristic> =
            device.characteristics().into_iter().find(|chr| {
                chr.properties == CharPropFlags::WRITE | CharPropFlags::WRITE_WITHOUT_RESPONSE
            });

        if characterics.is_none() {
            return Err(anyhow!("Failed to find D30 Bluetooth characteristics."));
        }

        Ok(Self {
            device,
            characteristic: characterics.unwrap(),
        })
    }

    pub async fn write(&self, data: &[u8]) -> Result<()> {
        self.device
            .write(&self.characteristic, data, WriteType::WithResponse)
            .await?;
        Ok(())
    }

    async fn d30_filter(p: &Peripheral, addr: &Option<BDAddr>) -> bool {
        let properties_res = p.properties().await;

        if let Err(e) = properties_res {
            warn!(
                "Error occured during get bluetooth device properties: {}",
                e
            );
            return false;
        }

        let properties = properties_res.unwrap();
        debug!("Found device: {:?}", properties);
        if properties.is_none() {
            return false;
        }

        let properties = properties.unwrap();

        // If execution reaches here, result is Ok, and you can unwrap or expect the value
        let local_name = properties.local_name.unwrap_or_default();
        debug!("Found BLE device: {}, {:?}", local_name, properties.address);

        if let Some(d30_addr) = addr {
            if properties.address == *d30_addr {
                return true;
            }
        } else if local_name == "D30" {
            return true;
        }

        false
    }

    async fn scan(central: Adapter, addr: Option<BDAddr>) -> Result<Peripheral> {
        #[cfg(debug_assertions)]
        {
            let adapter = match central.adapter_info().await {
                Ok(s) => s,
                Err(e) => {
                    debug!("Could not read adapter info: {:?}", e);
                    "".to_string()
                }
            };
            debug!("Scanning D30 from adapter: {:?}", adapter);

            let central_state = central.adapter_state().await.unwrap();
            debug!("CentralState: {:?}", central_state);
        }

        let mut events = central.events().await?;
        info!("Scanning Bluetooth devices");
        central.start_scan(ScanFilter::default()).await?;

        while let Some(event) = events.next().await {
            match event {
                CentralEvent::DeviceDiscovered(id) => {
                    let peripheral = central.peripheral(&id).await?;
                    if D30::d30_filter(&peripheral, &addr).await {
                        central.stop_scan().await?;
                        return Ok(peripheral);
                    }
                }
                _ => {}
            }
        }

        central.stop_scan().await?;
        Err(anyhow!("Could not find D30."))
    }

    async fn find_device(config: &Config) -> Result<Peripheral> {
        let manager = Manager::new().await?;
        info!("Searching for Bluetooth adapters");
        let adapters = manager.adapters().await?;

        if adapters.is_empty() {
            return Err(anyhow!("Unable to find any adapters."));
        }

        let adapter = adapters.into_iter().nth(0).unwrap();

        let addr = config.get_addr()?;
        let handle = spawn(async move { D30::scan(adapter, addr).await });
        let time_limit = Duration::from_secs(5);

        let d30 = timeout(time_limit, handle).await;
        match d30 {
            Ok(Ok(Ok(device))) => Ok(device),
            Ok(Ok(Err(e))) => Err(e),
            Ok(Err(join_error)) => Err(anyhow!("Task panicked or cancelled: {:?}", join_error)),
            Err(_) => Err(anyhow!(
                "Could not find D30: Timeout occurred after {:?}",
                time_limit
            )),
        }
    }
}
