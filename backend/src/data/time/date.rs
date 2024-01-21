use std::ops::Sub;

use super::duration::Duration;
use chrono::{Datelike, NaiveDateTime, Timelike};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct Date {
    pub year: u16,
    pub ordinal: u16,
    pub duration: Duration,
}

impl From<NaiveDateTime> for Date {
    fn from(value: NaiveDateTime) -> Self {
        Self {
            year: value.year() as u16,
            ordinal: value.ordinal0() as u16,
            duration: Duration::new(value.hour() as u8, value.minute() as u8),
        }
    }
}

impl Sub for Date {
    type Output = Duration;
    fn sub(self, rhs: Self) -> Self::Output {
        self.duration - rhs.duration
    }
}
