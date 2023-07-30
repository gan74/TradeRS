use serde_json::Value;

#[derive(Debug, PartialEq)]
pub enum MountType {
    Unknown(String),
    SensorArray,
    Surveyor,
    MiningLaser,
}

impl MountType {
    pub fn from_json(value: &Value) -> MountType {
        match value["symbol"].as_str().unwrap() {
            "MOUNT_SENSOR_ARRAY_I" => MountType::SensorArray,
            "MOUNT_SURVEYOR_I" => MountType::Surveyor,
            "MOUNT_MINING_LASER_I" => MountType::MiningLaser,

            unknown =>  MountType::Unknown(unknown.to_string()),
        }
    }
}

