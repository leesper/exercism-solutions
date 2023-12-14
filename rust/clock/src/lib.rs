use std::fmt::{Display, Result, Formatter};
use std::cmp::PartialEq;

#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

const HOURS_PER_DAY: i32 = 24;
const MINUTES_PER_HOUR: i32 = 60;

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let (adjusted_minutes, calculated_hours) = Self::adjust_minutes(minutes);

        let adjusted_hours = if (hours + calculated_hours) >= 0 { 
            (hours + calculated_hours) % HOURS_PER_DAY
        } else {
            (hours + calculated_hours) + ((hours + calculated_hours).abs() / HOURS_PER_DAY + 1) * HOURS_PER_DAY
        };

        Clock { 
            hours: adjusted_hours, 
            minutes: adjusted_minutes, 
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        todo!("Add {minutes} minutes to existing Clock time");
    }

    fn adjust_minutes(raw_minutes: i32) -> (i32, i32) {
        let (adjusted_minutes, calculated_hours) = if raw_minutes >= 0 {
            (raw_minutes % MINUTES_PER_HOUR, raw_minutes / MINUTES_PER_HOUR)
        } else {
            let round_up = if raw_minutes.abs() % MINUTES_PER_HOUR != 0 {1} else {0};
            (
                raw_minutes + (raw_minutes.abs() / MINUTES_PER_HOUR + round_up) * MINUTES_PER_HOUR, 
                -(raw_minutes.abs() / MINUTES_PER_HOUR + round_up)
            )
        };
        (adjusted_minutes, calculated_hours)
    }
}

impl Display for Clock {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}
