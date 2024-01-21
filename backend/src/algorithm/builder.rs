use crate::data::reservation::Reservation;

use crate::schedule::{day::Day, slot::Slot};

#[derive(Clone, Default)]
pub struct DayBuilder {
    slots: [Slot; 5],
    money: u64,
}

impl DayBuilder {
    pub fn build(to_take: &[Reservation]) -> Day {
        let mut day = Self::default();
        Self::default().best_recursive(to_take, &mut day);
        day.into_day()
    }
    fn best_recursive(&self, mut to_take: &[Reservation], best: &mut DayBuilder) {
        let mut new_comb = self.clone();
        if self.money + to_take.len() as u64 * 50 < best.money {
            return;
        }
        loop {
            let Some(taken) = to_take.first() else {
                if self.money > best.money {
                    *best = new_comb;
                    println!("{}", self.money);
                }
                return;
            };
            if let Some(slot) = new_comb.slots.iter_mut().find(|r| r.can_accept(&taken)) {
                slot.push(taken.clone());
                new_comb.money += taken.vehicle_type.get_price();
                break;
            }
            to_take = &to_take[1..];
        }

        for i in 0..to_take.len() {
            new_comb.best_recursive(&to_take[i..], best);
        }
    }
    fn into_day(self) -> Day {
        let mut day = Day::default();
        day.slots[0..5].clone_from_slice(&self.slots);
        day
    }
}
