use crate::data::reservation::Reservation;
use crate::schedule::slot::Slot;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct Day {
    pub slots: [Slot; 10],
    pub refused: Vec<Reservation>,
}

impl Day {
    pub fn push_walkin(&mut self, reservation: Reservation) {
        if reservation.is_walkin() {
            if let Some(slot) = self.slots.get_mut(5 + reservation.vehicle_type as usize) {
                if slot.can_accept(&reservation) {
                    slot.push(reservation);
                    return;
                }
            }
        }
        if let Some(slot) = self.slots[0..5]
            .iter_mut()
            .find(|r| r.can_accept(&reservation))
        {
            slot.push(reservation);
            return;
        }
        self.refused.push(reservation);
    }
}
