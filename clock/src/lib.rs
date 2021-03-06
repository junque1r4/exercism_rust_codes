use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let mut new_hours = hours;
        let mut new_minutes = minutes;

        match minutes >= 60 && minutes >= 0 {
            true => {
                while new_minutes >= 60 {
                    new_minutes -= 60;
                    new_hours += 1;
                }
            }
            false => {
                while new_minutes < 0 {
                    new_minutes += 60;
                    new_hours -= 1;
                }
            }
        }

        match new_hours > 23 {
            true => {
                while new_hours > 23 || new_hours == 24{
                    new_hours -= 24;
                }
            }
            false => {
                while new_hours < 0 {
                    new_hours += 24
                }
            }
        }

        Clock {
            hours: new_hours,
            minutes: new_minutes,    
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let mut new_minutes = self.minutes + minutes;
        let mut new_hours = self.hours;

        match new_minutes >= 60 {
            true => {
                while new_minutes >= 60 {
                    new_minutes -= 60;
                    new_hours += 1;
                }
                Clock::new(new_hours, new_minutes)
            }
            false => {
                while new_minutes < 0 {
                    new_minutes += 60;
                    new_hours -= 1;
                }
                Clock::new(new_hours, new_minutes)
            }
        }
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}
