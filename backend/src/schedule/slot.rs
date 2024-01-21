use crate::data::{reservation::Reservation, time::duration::Duration};
use derive_more::{Deref, DerefMut};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deref, DerefMut, Serialize, Deserialize, Clone)]
pub struct Slot {
    schedule: Vec<Reservation>,
}

impl Slot {
    pub fn can_accept(&self, reservation: &Reservation) -> bool {
        let Some(last) = self.schedule.last() else {
            return false;
        };
        !last.overlap(reservation)
    }

    pub fn end_time(&self) -> Duration {
        let Some(last) = self.schedule.last() else {
            return Duration::hours(7);
        };
        last.reservation_date.duration + last.vehicle_type.get_time()
    }

    pub fn len_served(&self) -> [usize; 5] {
        let mut served = [0; 5];
        for reservation in &self.schedule {
            served[reservation.vehicle_type as usize] += 1;
        }
        served
    }

    pub fn get_revenue(&self) -> u64 {
        self.schedule
            .iter()
            .map(|r| r.vehicle_type.get_price())
            .sum()
    }
}
