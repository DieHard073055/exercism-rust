use std::fmt;

#[derive(PartialEq, Debug)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let mut total_hours = hours;
        let mut total_minutes = minutes;
        
        if total_minutes == 60{
            total_minutes = 0;
            total_hours += 1;
        }
        
        else if total_minutes > 59 {
            let left_over_hours = total_minutes / 60;
            total_minutes = total_minutes - (left_over_hours * 60);
            total_hours += left_over_hours;
        }
        else if total_minutes < 0 {
            while total_minutes < 0 {
                if total_minutes < 0 {
                    total_minutes += 60;
                    total_hours -= 1;
                }
            }
        }
        
        if total_hours >= 24 {
            while total_hours >= 24 {
                total_hours -= 24;
            }

        }
        else if total_hours < 0 {
            while total_hours < 0 {
                total_hours = 24 - total_hours.abs() 
            }
        }
        
        Clock { hours: total_hours, minutes: total_minutes }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(self.hours,self.minutes + minutes)
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}
