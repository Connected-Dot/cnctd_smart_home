pub struct Govee;

impl Govee {
    pub get_devices(api_key: &str) -> Vec<SmartDevice> {
        let govee_client = GoveeClient::new();
		match govee_client.get_devices().await {
			Ok(res) => {
				let devices = res.data.unwrap();
				println!("Govee devices: {:?}", devices);
				// for device in devices.devices {
				// 	device.
				// }
			},
			Err(e) => println!("Error: {}", e),
		}	
    }
}