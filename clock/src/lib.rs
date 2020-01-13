use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}

impl Clock {
    fn normalize_time(hours: i32, minutes: i32) -> (i32, i32) {
        let mut final_hours = if minutes >= 60 {
            (hours + minutes / 60) % 24
        } else if minutes < -60 {
            (hours - 1 + minutes / 60) % 24 
        } else if minutes < 0 {
            (hours - 1) % 24
        } else {
            hours % 24
        };
        if final_hours < 0 {
            final_hours += 24;
        }

        let mut final_minutes = minutes % 60;
        if final_minutes < 0 {
            final_minutes += 60;
        }

        (final_hours, final_minutes)
    }

    pub fn new(hours: i32, minutes: i32) -> Self {
        let (final_hours, final_minutes) = Clock::normalize_time(hours, minutes);
        Clock { hours : final_hours, minutes : final_minutes }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        if minutes == 0 {
            Clock { hours : self.hours, minutes : self.minutes}
        } else {
            let (final_hours, final_minutes) = Clock::normalize_time(self.hours, self.minutes + minutes);
            Clock { hours : final_hours, minutes : final_minutes}
        }
    }
}
