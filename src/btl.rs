use core::time;
use std::thread;

use crate::config::Config;
use anyhow::{Context, Result, anyhow};
use btleplug::api::{
    BDAddr, Central, CharPropFlags, Characteristic, Manager as _, Peripheral as _, ScanFilter,
    WriteType,
};
use btleplug::platform::{Adapter, Manager, Peripheral};

pub struct D30 {
    device: Peripheral,
    characteristic: Characteristic,
}

impl D30 {
    pub async fn new(config: &Config) -> Result<Self> {
        let device = D30::find_device(config.get_addr()?).await?;

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

    async fn d30_filter(central: &Adapter, addr: Option<BDAddr>) -> Result<Peripheral> {
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
        }

        info!("Scanning Bluetooth devices");
        central.start_scan(ScanFilter::default()).await?;

        //TODO: change to event driven api.
        let duration = time::Duration::from_secs(2);
        info!("Waiting for {} seconds", duration.as_secs_f32());
        thread::sleep(duration);

        for p in central.peripherals().await? {
            let properties_res = p.properties().await;

            if let Err(e) = properties_res {
                warn!(
                    "Error occured during get bluetooth device properties: {}",
                    e
                );
                continue;
            }

            let properties = properties_res.unwrap();
            debug!("Found device: {:?}", properties);
            if properties.is_none() {
                continue;
            }

            let properties = properties.unwrap();

            // If execution reaches here, result is Ok, and you can unwrap or expect the value
            let local_name = properties.local_name.unwrap_or_default();
            debug!("Found BLE device: {}, {:?}", local_name, properties.address);

            if let Some(addr) = addr {
                if properties.address == addr {
                    return Ok(p);
                }
            } else if local_name == "D30" {
                return Ok(p);
            }
        }

        Err(anyhow!("Could not find D30."))
    }

    async fn find_device(addr: Option<BDAddr>) -> Result<Peripheral> {
        let manager = Manager::new().await?;
        info!("Searching for Bluetooth adapters");
        let adapters = manager.adapters().await?;

        if adapters.is_empty() {
            return Err(anyhow!("Unable to find any adapters."));
        }

        for adapter in adapters {
            if let Ok(a) = D30::d30_filter(&adapter, addr).await {
                return Ok(a);
            }
        }

        Err(anyhow!("Could not find D30 from any adapter."))
    }
}
