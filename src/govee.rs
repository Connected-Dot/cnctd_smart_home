use govee_api::GoveeClient;

use crate::{DeviceType, SmartDevice, SupportedFeature};

pub struct Govee;

impl Govee {
    pub async fn get_devices(api_key: &str) -> Vec<SmartDevice> {
        let govee_client = GoveeClient::new(api_key);
		let mut smart_devices: Vec<SmartDevice> = vec![];

		match govee_client.get_devices().await {
			Ok(res) => {
				let devices = res.data.unwrap();
				println!("Govee devices: {:?}", devices);
				for device in devices.devices {
					let mut features: Vec<SupportedFeature> = vec![];
					for feature in device.supportCmds {
						if &feature == "turn" { features.push(SupportedFeature::OnOff) }
						if &feature == "brightness" { features.push(SupportedFeature::Brightness) }
						if &feature == "color" { features.push(SupportedFeature::Color) }
						if &feature == "colorTem" { features.push(SupportedFeature::ColorTemp) }
					}
					match govee_client.get_device_state(&device.device, &device.model).await {
						Ok(state) => {
							match DeviceType::get_type_from_str(&device.model) {
								Ok(device_type) => {
									let smart_device = SmartDevice::new(&device.deviceName, None, features, device_type);
									smart_devices.push(smart_device);
								}
								Err(e) => println!("Error: {}", e),
							}
						}
						Err(e) => println!("Error: {:?}", e)
					}
					
					
				}


			},
			Err(e) => println!("Error: {}", e),
		}	

		smart_devices
    }
}