use derive_more::Deref;
use derive_more::DerefMut;
use derive_more::Sub;
use serde::Deserialize;
use serde::Serialize;

#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    Deref,
    DerefMut,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Sub,
)]
pub struct Duration(i16);

impl Duration {
    pub fn new(hour: u8, minutes: u8) -> Self {
        Self(hour as i16 * 60 + minutes as i16)
    }
    pub fn minutes(minutes: u8) -> Self {
        Self(minutes as i16)
    }
    pub fn hours(hours: u8) -> Self {
        Self(hours as i16 * 60)
    }
    pub fn hour(&self) -> i16 {
        self.0 / 60
    }
}
