use backend::{
    algorithm::builder::DayBuilder,
    data::reservations::Reservations,
    schedule::{year::Year, Schedule},
};

fn main() {
    let reservations = Reservations::new_from_file("datafile.csv").unwrap();
    let year = Year::new(reservations.clone());
    let mut schedule = Schedule::default();
    schedule.add_all_fifo(reservations.to_vec());
    let fifo_comb = schedule.get_day(332).unwrap();
    let mut best_comb = DayBuilder::build(&year.get_day(332).reservation);
    for walkin in &year.get_day(332).walkin {
        best_comb.push_walkin(walkin.clone());
    }
    println!(
        "{:#?}",
        year.get_day(332).reservation.len() + year.get_day(332).walkin.len()
    );
    println!(
        "{:#?}",
        fifo_comb.get_vehicules_refused().iter().sum::<usize>()
            + fifo_comb.get_vehicules_served().iter().sum::<usize>()
    );
    println!(
        "{:#?}",
        best_comb.get_vehicules_refused().iter().sum::<usize>()
            + best_comb.get_vehicules_served().iter().sum::<usize>()
    );
    // println!("{:#?}", fifo_comb.get_day_info());
    // println!("{:#?}", best_comb.get_day_info());
}
