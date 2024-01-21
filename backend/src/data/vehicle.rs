use super::time::duration::Duration;
use serde::{Deserialize, Serialize};

#[repr(u8)]
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum VehicleType {
    CompactCars = 0,
    MediumCars = 1,
    FullSizeCars = 2,
    Class1Trucks = 3,
    Class2Trucks = 4,
}

impl VehicleType {
    pub fn from_string(s: &str) -> Self {
        match s {
            "compact" => Self::CompactCars,
            "medium" => Self::MediumCars,
            "full-size" => Self::FullSizeCars,
            "class 1 truck" => Self::Class1Trucks,
            "class 2 truck" => Self::Class2Trucks,
            _ => panic!("Invalid vehicle type"),
        }
    }

    pub fn get_price(&self) -> u64 {
        match self {
            VehicleType::CompactCars => 150,
            VehicleType::MediumCars => 150,
            VehicleType::FullSizeCars => 150,
            VehicleType::Class1Trucks => 250,
            VehicleType::Class2Trucks => 750,
        }
    }

    pub fn get_time(&self) -> Duration {
        match self {
            VehicleType::CompactCars => Duration::minutes(30),
            VehicleType::MediumCars => Duration::minutes(30),
            VehicleType::FullSizeCars => Duration::minutes(30),
            VehicleType::Class1Trucks => Duration::hours(1),
            VehicleType::Class2Trucks => Duration::hours(2),
        }
    }
}
