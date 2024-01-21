use serde::Serialize;
use utoipa::ToSchema;

use crate::algorithm::day::Day;

#[derive(Debug, Serialize, ToSchema)]
pub struct DayInfo {
    revenue: u64,
    delta_revenue: i64,
    revenue_miss: u64,
    delta_revenue_miss: i64,
    clients_served: usize,
    delta_clients_served: isize,
    vehicules_served: [usize; 5],
    delta_vehicules_served: [isize; 5],
    vehicules_decline: [usize; 5],
    delta_vehicules_decline: [isize; 5],
}

impl Day {
    pub fn get_day_info(&self, previous_day: &Day) -> DayInfo {
        let revenue = self.get_revenue();
        let delta_revenue = previous_day.get_revenue() as i64 - revenue as i64;

        let revenue_miss = self.get_revenue_miss();
        let delta_revenue_miss = previous_day.get_revenue_miss() as i64 - revenue_miss as i64;

        let clients_served = self.clients_served();
        let delta_clients_served = previous_day.clients_served() as isize - clients_served as isize;

        let mut delta_vehicules_served = [0; 5];
        let vehicules_served = self.get_vehicules_served();
        let previous_vehicule_served = previous_day.get_vehicules_served();
        for i in 0..5 {
            delta_vehicules_served[i] =
                vehicules_served[i] as isize - previous_vehicule_served[i] as isize;
        }

        let mut delta_vehicules_decline = [0; 5];
        let vehicules_decline = self.get_vehicules_refused();
        let previous_vehicule_decline = previous_day.get_vehicules_refused();
        for i in 0..5 {
            delta_vehicules_decline[i] =
                vehicules_decline[i] as isize - previous_vehicule_decline[i] as isize;
        }
        DayInfo {
            clients_served,
            delta_clients_served,
            revenue,
            delta_revenue,
            revenue_miss,
            delta_revenue_miss,
            vehicules_served,
            delta_vehicules_served,
            vehicules_decline,
            delta_vehicules_decline,
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
