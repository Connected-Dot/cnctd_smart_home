use std::{fmt, thread::sleep, time::Duration};

use tplinker::{
    discovery::discover,
    devices::{Device, RawDevice, HS100, HS105, KL110, LB110, LB120},
    capabilities::{DeviceActions, Switch},	
};

use crate::{DeviceStatus, DeviceType, Model, SmartDevice, SupportedFeature};

pub struct TPLink;

impl TPLink {
    pub async fn get_devices() -> Vec<SmartDevice> {
		let mut smart_devices: Vec<SmartDevice> = vec![];
        for (addr, data) in discover().unwrap() {
			let sysinfo = data.sysinfo();
			let mut features: Vec<SupportedFeature> = vec![];
			features.push(SupportedFeature::OnOff);
			
			if sysinfo.is_dimmable() { features.push(SupportedFeature::Brightness) }
			if sysinfo.is_color() { features.push(SupportedFeature::Color) }
			if sysinfo.is_variable_color_temp() { features.push(SupportedFeature::ColorTemp) }

			// println!("{}\t{}\t{}\t{}\n{:?}", addr, sysinfo.alias, sysinfo.hw_type, sysinfo.model, data.smartlife.);
			println!("{:?}", sysinfo);
			match DeviceType::get_type_from_str(&sysinfo.model) {
				Ok(device_type) => {
					let smart_device = SmartDevice::new(&sysinfo.alias, Some(addr), features, device_type);
					if !smart_devices.iter().any(|d| d.addr == Some(addr)) {
						let status = DeviceStatus {
							on_off: sysinfo.relay_state,
							hue: todo!(),
							saturation: todo!(),
							color_temp: todo!(),
							brightness: todo!(),
						};
						smart_devices.push(smart_device);
					}
				}
				Err(e) => println!("Error {}", e),
			}
			

	  	}
		println!("tplink length: {}", smart_devices.len());
		smart_devices
    }

}