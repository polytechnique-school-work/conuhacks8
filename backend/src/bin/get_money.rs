use backend::{
    data::{reservation::Reservation, reservations::Reservations},
    schedule::{year::Year, Schedule},
};
use chrono::Timelike;

fn main() {
    let reservations = Reservations::new_from_file("datafile.csv");
    let year = Year::new(reservations.clone());
    let mut schedule = Schedule::default();
    schedule.add_all_fifo(reservations);
    // for i in 0..365 {
    //     println!("{} {}", i, year.get_day(i).reservation.len());
    // }
    // let best_comb = DayBuilder::build(&year.get_day(332).reservation);
    let mut reservation = year.get_day(332).reservation.clone();
    reservation.sort_unstable_by_key(|r| r.reservation_date);
    println!(
        "{:#?}",
        reservation
            .iter()
            .filter(|r| r.reservation_date.hour() > 5)
            .cloned()
            .collect::<Vec<Reservation>>()
    );
    // println!("{:#?}", best_comb.get_day_info());
    // println!("{:#?}", schedule.get_day(332).unwrap().get_day_info());
}
