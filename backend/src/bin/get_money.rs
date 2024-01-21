use backend::{
    algorithm::builder::DayBuilder, data::reservations::Reservations, schedule::year::Year,
};

fn main() {
    let reservations = Reservations::new_from_file("datafile.csv");
    let year = Year::new(reservations.clone());
    let best_comb = DayBuilder::build(&year.get_day(332).reservation);
    println!("{:#?}", best_comb);
}
