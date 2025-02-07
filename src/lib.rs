use std::{fmt, net::SocketAddr};

use govee::Govee;
use serde::{Serialize, Deserialize, Serializer, Deserializer, de::{self, Visitor}};
use tplink::TPLink;
use anyhow::anyhow;

pub mod tplink;
pub mod govee;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct SmartDevice {
    pub name: String,
    pub addr: Option<SocketAddr>,
    pub supported_features: Vec<SupportedFeature>,
    pub device_type: DeviceType,
    pub status: DeviceStatus,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct DeviceStatus {
    pub on_off: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hue: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub saturation: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color_temp: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brightness: Option<u16>,
}

#[derive(Debug, PartialEq)]
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

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum DeviceType {
    TPLinkBulb(Model),
    TPLinkSwitch(Model),
    GoveeLight(Model),
}
  
impl SmartDevice {
    pub fn new(name: &str, addr: Option<SocketAddr>, supported_features: Vec<SupportedFeature>, device_type: DeviceType, status: DeviceStatus) -> Self {
        Self {
            name: name.into(),
            addr,
            supported_features,
            device_type,
            status,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum Model {
    KL110,
    HS100,
    HS105,
    LB120,
    LB110,
    H6159,
}

pub struct SmartHome;

impl SmartHome {
    pub async fn get_devices(govee_api_key: &str) -> Vec<SmartDevice> {
        let mut devices = TPLink::get_devices().await;
        let mut govee_devices = Govee::get_devices(govee_api_key).await;
        devices.append(&mut govee_devices);

        println!("devices length: {}", devices.len());
        devices
    }

}

impl DeviceType {
    pub fn get_type_from_str(device_str: &str) -> anyhow::Result<DeviceType> {
        match device_str {
            "KL130(US)" | "KL110(US)" => Ok(DeviceType::TPLinkBulb(Model::KL110)),
            "HS100(US)" => Ok(DeviceType::TPLinkSwitch(Model::HS100)),
            "HS105(US)" => Ok(DeviceType::TPLinkSwitch(Model::HS105)),
            "LB120(US)" => Ok(DeviceType::TPLinkBulb(Model::LB120)),
            "LB100(US)" | "LB110(US)" | "LB200(E26)" => Ok(DeviceType::TPLinkBulb(Model::LB110)),
            "H6159" => Ok(DeviceType::GoveeLight(Model::H6159)),
            &_ => Err(anyhow!("device not supported"))
        }
    }

}