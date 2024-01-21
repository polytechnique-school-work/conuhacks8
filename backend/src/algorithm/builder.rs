use super::day::Day;
use crate::data::{
    reservation::Reservation, reservations::Reservations, time::duration::Duration,
    vehicle::VehicleType,
};

#[derive(Clone, Default)]
pub struct DayBuilder {
    cars: Reservations,
    truck1: Reservations,
    truck2: Reservations,
}

impl DayBuilder {
    fn new(to_take: &[Reservation]) -> Self {
        let mut day_builder = Self::default();
        for reservation in to_take {
            match reservation.vehicle_type {
                VehicleType::CompactCars | VehicleType::MediumCars | VehicleType::FullSizeCars => {
                    day_builder.cars.push(reservation.clone())
                }
                VehicleType::Class1Trucks => day_builder.truck1.push(reservation.clone()),
                VehicleType::Class2Trucks => day_builder.truck2.push(reservation.clone()),
            }
        }
        let compare = |a: &Reservation, b: &Reservation| {
            a.reservation_date
                .cmp(&b.reservation_date)
                .then(a.call_date.cmp(&b.call_date))
                .reverse()
        };
        day_builder.cars.sort_unstable_by(compare);
        day_builder.truck1.sort_unstable_by(compare);
        day_builder.truck2.sort_unstable_by(compare);
        day_builder
    }
    fn select_best(&mut self, t0: Duration, refused: &mut Vec<Reservation>) -> Option<Reservation> {
        self.cars.remove_all_before(t0, refused);
        self.truck1.remove_all_before(t0, refused);
        self.truck2.remove_all_before(t0, refused);
        if let Some(t2) = self.truck2.last_duration() {
            let dt2 = t2 - t0;
            if dt2 < Duration::minutes(30) {
                return self.truck2.pop();
            }
            if dt2 <= Duration::minutes(180) {
                if let Some(tv1) = self.cars.last_duration() {
                    if t2 - tv1 >= Duration::minutes(30) {
                        return self.cars.pop();
                    }
                }
                return self.truck2.pop();
            }
        }
        if let Some(t1) = self.truck1.last_duration() {
            if let Some(tv1) = self.cars.last_duration() {
                if tv1 - t1 >= Duration::minutes(12) && t1 - t0 < Duration::minutes(120) {
                    return self.truck1.pop();
                } else if let Some(tv2) = self
                    .cars
                    .iter()
                    .find(|c| c.reservation_date.duration >= tv1 + Duration::minutes(30))
                    .map(|tv2| tv2.reservation_date.duration)
                {
                    let dtv2 = tv2 - t0;
                    let dt1 = t1 - t0;
                    if dt1 + Duration::minutes(30) < dtv2 && dt1 < Duration::minutes(120) {
                        return self.truck1.pop();
                    }
                }
            } else if t1 - t0 < Duration::minutes(120) {
                return self.truck1.pop();
            }
        }
        if let Some(tv1) = self.cars.last_duration() {
            if tv1 - t0 + Duration::minutes(30) < Duration::minutes(180) {
                return self.cars.pop();
            }
        }
        None
    }
    fn generate_best(mut self) -> Day {
        let mut day = Day::default();
        let mut best_slot = day.slots[0..5]
            .iter_mut()
            .min_by_key(|s| s.end_time())
            .unwrap();
        let mut t0 = best_slot.end_time();
        while t0 < Duration::hours(19) {
            let best = self.select_best(best_slot.end_time(), &mut day.refused);
            if let Some(best) = best {
                best_slot.push(best);
                best_slot = day.slots[0..5]
                    .iter_mut()
                    .min_by_key(|s| s.end_time())
                    .unwrap();
                *t0 = *best_slot.end_time();
            } else {
                *t0 += 30;
            }
        }
        day.refused.extend_from_slice(&self.cars);
        day.refused.extend_from_slice(&self.truck1);
        day.refused.extend_from_slice(&self.truck2);
        day
    }
    pub fn build(to_take: &[Reservation]) -> Day {
        let day = Self::new(to_take);
        day.generate_best()
    }
}
