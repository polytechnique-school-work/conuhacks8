use chrono::{Duration, NaiveDateTime};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Reservation {
    pub id: Uuid,
    pub call_date: NaiveDateTime,
    pub reservation_date: NaiveDateTime,
    pub vehicle_type: VehicleType,
}

impl Reservation {
    pub fn new(
        call_date: NaiveDateTime,
        reservation_date: NaiveDateTime,
        vehicle_type: VehicleType,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            call_date,
            reservation_date,
            vehicle_type,
        }
    }

    pub fn new_from_string(line: &str) -> Self {
        let mut split = line.split(',');
        let call_date =
            NaiveDateTime::parse_from_str(split.next().unwrap(), "%Y-%m-%d %H:%M").unwrap();
        let reservation_date =
            NaiveDateTime::parse_from_str(split.next().unwrap(), "%Y-%m-%d %H:%M").unwrap();
        let vehicle_type = VehicleType::from_string(split.next().unwrap());
        Self {
            id: Uuid::new_v4(),
            call_date,
            reservation_date,
            vehicle_type,
        }
    }

    pub fn overlap(&self, reservation: &Reservation) -> bool {
        let reservation_time = std::cmp::max(
            self.vehicle_type.get_time(),
            reservation.vehicle_type.get_time(),
        );
        let delta = self.reservation_date - reservation.reservation_date;
        delta.abs() < reservation_time
    }
}
