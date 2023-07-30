use serde_json::Value;

use crate::utils::*;
use crate::outfitting::*;

#[derive(Debug)]
pub enum ShipStatus {
    Docked,
    InTransit,
    InOrbit,
}

#[derive(Debug)]
pub struct Ship {
    symbol: String,
    waypoint: String,

    status: ShipStatus,

    fuel: i64,
    max_fuel: i64,

    crew: i64,
    max_crew: i64,

    inventory: ShipInventory,

    mounts: Vec<MountType>,
}

impl Ship {
    pub fn from_json(value: &Value) -> Ship {
        let nav = &value["nav"];
        let fuel = &value["fuel"];
        let crew = &value["crew"];

        let mounts = match &value["mounts"] {
            Value::Array(arr) => arr.into_iter().map(|val| MountType::from_json(val)).collect::<Vec<_>>(),
            _ => Vec::new(),
        };

        Ship{
            symbol: as_string(&value["symbol"]),
            waypoint: as_string(&nav["waypointSymbol"]),

            status: ship_status_from_json(&nav["status"]),

            fuel: fuel["current"].as_i64().unwrap(),
            max_fuel: fuel["capacity"].as_i64().unwrap(),

            crew: crew["current"].as_i64().unwrap(),
            max_crew: crew["capacity"].as_i64().unwrap(),

            inventory: ShipInventory::from_json(&value["cargo"]),

            mounts: mounts,
        }
    }

    pub fn name(&self) -> &str {
        &self.symbol
    }

    pub fn has_mounted(&self, mount: &MountType) -> bool {
        self.mounts.contains(mount)
    }

    pub fn is_drone(&self) -> bool {
        self.max_crew == 0
    }
}


#[derive(Debug)]
pub struct ShipInventory {
    capacity: i64,
    used: i64,

    inventory: Vec<(String, i64)>,
}

impl ShipInventory {
    pub fn from_json(value: &Value) -> ShipInventory {
        let inventory = match &value["inventory"] {
            Value::Array(arr) => arr.into_iter().map(|val| (as_string(&val["symbol"]), val["units"].as_i64().unwrap())).collect::<Vec<_>>(),
            _ => Vec::new(),
        };

        ShipInventory{
            capacity: value["capacity"].as_i64().unwrap(),
            used: value["units"].as_i64().unwrap(),

            inventory: inventory,
        }
    }
}




fn ship_status_from_json(value: &Value) -> ShipStatus {
    match value.as_str().unwrap() {
        "DOCKED" => ShipStatus::Docked,
        "IN_TRANSIT" => ShipStatus::InTransit,
        "IN_ORBIT" => ShipStatus::InOrbit,
        v => panic!("Unknown enum value {:?}", v),
    }
}