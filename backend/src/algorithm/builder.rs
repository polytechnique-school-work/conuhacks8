use super::day::Day;
use crate::data::{reservation::Reservation, time::duration::Duration, vehicle::VehicleType};

#[derive(Clone, Default)]
pub struct DayBuilder {
    cars: Vec<Reservation>,
    truck1: Vec<Reservation>,
    truck2: Vec<Reservation>,
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
    fn select_best(&mut self, t0: Duration) -> Option<Reservation> {
        while let Some(car) = self.cars.last() {
            if car.reservation_date.duration < t0 {
                self.cars.pop();
            } else {
                break;
            }
        }
        while let Some(truck1) = self.truck1.last() {
            if truck1.reservation_date.duration < t0 {
                self.truck1.pop();
            } else {
                break;
            }
        }
        while let Some(truck2) = self.truck2.last() {
            if truck2.reservation_date.duration < t0 {
                self.truck2.pop();
            } else {
                break;
            }
        }
        if let Some(t2) = self.truck2.last().map(|t2| t2.reservation_date.duration) {
            let dt2 = t2 - t0;
            if dt2 < Duration::minutes(30) {
                return self.truck2.pop();
            }
            if dt2 <= Duration::minutes(180) {
                if let Some(tv1) = self.cars.last().map(|t2| t2.reservation_date.duration) {
                    if t2 - tv1 >= Duration::minutes(30) {
                        return self.cars.pop();
                    }
                }
                return self.truck2.pop();
            }
        }
        if let Some(t1) = self.truck1.last().map(|t2| t2.reservation_date.duration) {
            if let Some(tv1) = self.cars.last().map(|tv1| tv1.reservation_date.duration) {
                if tv1 - t1 >= Duration::minutes(12) && t1 - t0 < Duration::minutes(120) {
                    return self.truck1.pop();
                } else {
                    if let Some(tv2) = self
                        .cars
                        .iter()
                        .find(|c| c.reservation_date.duration >= tv1 + Duration::minutes(30))
                        .map(|tv2| tv2.reservation_date.duration)
                    {
                        let dtv2 = tv2 - t0;
                        let dt1 = t1 - t0;
                        if dt1 + Duration::minutes(30) < dtv2 && t1 - t0 < Duration::minutes(120) {
                            return self.truck1.pop();
                        }
                    }
                    return self.cars.pop();
                }
            } else {
                return self.truck1.pop();
            }
        }
        self.cars.pop()
    }
    fn generate_best(mut self) -> Day {
        let mut day = Day::default();
        let mut best_slot = day.slots[0..5]
            .iter_mut()
            .min_by_key(|s| s.end_time())
            .unwrap();
        while best_slot.end_time() < Duration::hours(19) {
            let best = self.select_best(best_slot.end_time());
            if let Some(best) = best {
                best_slot.push(best);
            } else {
                break;
            }

            best_slot = day.slots[0..5]
                .iter_mut()
                .min_by_key(|s| s.end_time())
                .unwrap();
        }
        day
    }
    pub fn build(to_take: &[Reservation]) -> Day {
        let day = Self::new(to_take);
        day.generate_best()
    }
}
