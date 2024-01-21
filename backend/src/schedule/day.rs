use super::slot::Slot;
use crate::data::reservation::Reservation;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct Day {
    slots: [Slot; 10],
    refused: Vec<Reservation>,
}

impl Day {
    pub fn push(&mut self, reservation: Reservation) {
        if let Some(slot) = self.slots[0..5]
            .iter_mut()
            .find(|r| r.can_accept(&reservation))
        {
            slot.push(reservation);
            return;
        }
        if reservation.is_walkin() {
            if let Some(slot) = self.slots[6..10]
                .iter_mut()
                .find(|r| r.can_accept(&reservation))
            {
                slot.push(reservation);
                return;
            }
        }
        self.refused.push(reservation);
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
