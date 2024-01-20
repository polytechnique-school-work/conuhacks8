use chrono::{Duration, NaiveDateTime};
use uuid::Uuid;

#[derive(Debug)]
pub enum VehicleType {
    CompactCars,
    MediumCars,
    FullSizeCars,
    Class1Trucks,
    Class2Trucks,
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

    pub fn get_price(&self) -> f64 {
        match self {
            VehicleType::CompactCars => 150.0,
            VehicleType::MediumCars => 150.0,
            VehicleType::FullSizeCars => 150.0,
            VehicleType::Class1Trucks => 250.0,
            VehicleType::Class2Trucks => 750.0,
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

#[derive(Debug)]
pub struct Reservation {
    pub id: Uuid,
    pub start_date: NaiveDateTime,
    pub end_date: NaiveDateTime,
    pub vehicle_type: VehicleType,
}

impl Reservation {
    pub fn new(
        start_date: NaiveDateTime,
        end_date: NaiveDateTime,
        vehicle_type: VehicleType,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            start_date,
            end_date,
            vehicle_type,
        }
    }

    pub fn new_from_string(line: &str) -> Self {
        let mut split = line.split(',');
        let start_date =
            NaiveDateTime::parse_from_str(split.next().unwrap(), "%Y-%m-%d %H:%M").unwrap();
        let end_date =
            NaiveDateTime::parse_from_str(split.next().unwrap(), "%Y-%m-%d %H:%M").unwrap();
        let vehicle_type = VehicleType::from_string(split.next().unwrap());
        Self {
            id: Uuid::new_v4(),
            start_date,
            end_date,
            vehicle_type,
        }
    }
}
