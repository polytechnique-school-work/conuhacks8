use serde::Serialize;
use utoipa::ToSchema;

use crate::algorithm::day::Day;

#[derive(Debug, Serialize, ToSchema)]
pub struct DayInfo {
    revenue: u64,
    revenue_miss: u64,
    clients_served: usize,
    vehicules_served: [usize; 5],
    vehicules_decline: [usize; 5],
}

impl Day {
    pub fn get_day_info(&self) -> DayInfo {
        DayInfo {
            clients_served: self.clients_served(),
            revenue: self.get_revenue(),
            revenue_miss: self.get_revenue_miss(),
            vehicules_served: self.get_vehicules_served(),
            vehicules_decline: self.get_vehicules_refused(),
        }
    }

    pub fn get_vehicules_refused(&self) -> [usize; 5] {
        let mut count = [0; 5];

        for reservation in &self.refused {
            count[reservation.vehicle_type as usize] += 1;
        }

        count
    }

    pub fn get_vehicules_served(&self) -> [usize; 5] {
        let mut count = [0; 5];

        for slot in &self.slots {
            let slot_count = slot.len_served();
            for i in 0..5 {
                count[i] += slot_count[i];
            }
        }
        count
    }

    pub fn clients_served(&self) -> usize {
        self.get_vehicules_served().into_iter().sum::<usize>()
    }

    pub fn get_revenue(&self) -> u64 {
        self.slots.iter().map(|s| s.get_revenue()).sum()
    }

    pub fn get_revenue_miss(&self) -> u64 {
        self.refused
            .iter()
            .map(|r| r.vehicle_type.get_price())
            .sum()
    }
}
