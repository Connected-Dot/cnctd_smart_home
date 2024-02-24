use std::{fmt, thread::sleep, time::Duration};

use tplinker::{
    discovery::discover,
    devices::{Device, RawDevice, HS100, HS105, KL110, LB110, LB120},
    capabilities::{DeviceActions, Switch},	
};
use serde::{Serialize, Deserialize, Serializer, Deserializer, de::{self, Visitor}};
use govee_api::GoveeClient;

pub mod tplink;
pub mod govee;

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
  
