use std::fmt;

const MINUTES_IN_DAY: i32 = 24 * 60;

pub struct Clock{
    minutes: i32,
}
fn normalize(minutes: i32) -> i32 {
    let mut m = minutes % MINUTES_IN_DAY;
    if m < 0 {
        m += MINUTES_IN_DAY;
    }
    m
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let total = hours * 60 + minutes;
        Clock {
            minutes: normalize(total),
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock { minutes:normalize(self.minutes + minutes), }
    }
}
impl PartialEq for Clock {
    fn eq(&self, other: &Self) -> bool {
        self.minutes == other.minutes
    }
}
