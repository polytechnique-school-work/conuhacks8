use backend::{
    data::reservations::Reservations,
    schedule::{year::Year, Schedule},
};

fn main() {
    let reservations = Reservations::new_from_file("datafile.csv");
    let year = Year::new(reservations.clone());
    let mut schedule = Schedule::default();
    schedule.add_all_fifo(reservations);
    // println!("{:#?}", year.get_day(280));
    println!("{:#?}", schedule.get_day(280).unwrap().get_day_info());
}
