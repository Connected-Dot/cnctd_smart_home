use std::{fmt, thread::sleep, time::Duration};

use tplinker::{
    discovery::discover,
    devices::{Device, RawDevice, HS100, HS105, KL110, LB110, LB120},
    capabilities::{DeviceActions, Switch},	
};
use serde::{Serialize, Deserialize, Serializer, Deserializer, de::{self, Visitor}};
use govee_api::GoveeClient;

#[derive(Serialize, Deserialize, Debug)]
pub struct SmartDevice {
    pub name: String,
    pub supported_features: Vec<SupportedFeature>,
    pub device_type: DeviceType,
}

#[derive(Debug)]
pub enum SupportedFeature {
    OnOff,
    Brightness,
    Color,
    ColorTemp,
}

impl Serialize for SupportedFeature {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let string = match *self {
            SupportedFeature::OnOff => "on_off",
            SupportedFeature::Brightness => "brightness",
            SupportedFeature::Color => "color",
            SupportedFeature::ColorTemp => "color_temp",
        };
        serializer.serialize_str(string)
    }
}

impl<'de> Deserialize<'de> for SupportedFeature {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(SupportedFeatureVisitor)
    }
}

struct SupportedFeatureVisitor;

impl<'de> Visitor<'de> for SupportedFeatureVisitor {
    type Value = SupportedFeature;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a string representing a supported feature")
    }

    fn visit_str<E>(self, value: &str) -> Result<SupportedFeature, E>
    where
        E: de::Error,
    {
        match value {
            "on_off" => Ok(SupportedFeature::OnOff),
            "brightness" => Ok(SupportedFeature::Brightness),
            "color" => Ok(SupportedFeature::Color),
            "color_temp" => Ok(SupportedFeature::ColorTemp),
            _ => Err(E::custom(format!("unknown feature: {}", value))),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum DeviceType {
    Bulb,
    Switch,
}
  
#[tokio::main]
async fn main() {
	loop {
		for (addr, data) in discover().unwrap() {
			let mut connected_devices: Vec<SmartDevice> = vec![];
			let sysinfo = data.sysinfo();
			println!("{}\t{}\t{}\t{}", addr, sysinfo.alias, sysinfo.hw_type, sysinfo.model);
			match &*sysinfo.model {
				"KL130(US)" | "KL110(US)" => {
					let device = KL110::from_addr(addr);
					println!("Device name: {}", device.sysinfo().unwrap().alias);
				}
				"HS100(US)" => {
					let device = HS100::from_addr(addr);
					println!("Device name: {}", device.sysinfo().unwrap().alias);
				}
				"HS105(US)" => {
					let device = HS105::from_addr(addr);
					println!("Device name: {}", device.sysinfo().unwrap().alias);
				}
				"LB120(US)" => {
					let device = LB120::from_addr(addr);
					println!("Device name: {}", device.sysinfo().unwrap().alias);
				}
				"LB100(US)" | "LB110(US)" | "LB200(E26)"=> {
					let device = LB110::from_addr(addr);
					println!("Device name: {}", device.sysinfo().unwrap().alias);
				}
				&_ => {
					println!("device not yet supported");
				}
			}
	  	}
		
		let govee_client = GoveeClient::new("77b141fb-e311-4c04-942b-d13a30ab4b33");
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

		sleep(Duration::from_secs(30))
	}
    
}