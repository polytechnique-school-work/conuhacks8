use serde::Serialize;
use utoipa::ToSchema;

use super::day::Day;

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
}
