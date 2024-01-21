use crate::data::reservation::Reservation;
use derive_more::{Deref, DerefMut};
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Clone, Debug, Default, Deref, DerefMut)]
pub struct Reservations {
    pub reservations: Vec<Reservation>,
}

impl Reservations {
    pub fn new() -> Self {
        Reservations {
            reservations: Vec::new(),
        }
    }

    pub fn new_from_file(filename: &str) -> Self {
        let mut reservations = Vec::new();
        let file = File::open(filename).expect("Unable to open file");
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let line = line.unwrap();
            let reservation = Reservation::new_from_string(&line);
            reservations.push(reservation);
        }
        reservations.sort_unstable_by_key(|r| r.call_date);
        Self { reservations }
    }

    pub fn into_iter(self) -> impl Iterator<Item = Reservation> {
        self.reservations.into_iter()
    }
}
