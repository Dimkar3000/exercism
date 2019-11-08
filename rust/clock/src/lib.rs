#[derive(Debug)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Clock {
        let mut h = hours;
        let mut m = minutes;
        h += m / 60;
        m %= 60;

        if m < 0 {
            m += 60;
            h -= 1;
        }
        h %= 24;
        if h < 0 {
            h += 24;
        }

        Clock {
            hours: h,
            minutes: m,
        }
    }

    pub fn to_string(&self) -> String {
        std::fmt::format(format_args!("{:02}:{:02}", self.hours, self.minutes))
    }

    pub fn add_minutes(&self, minutes: i32) -> Clock {
        Clock::new(self.hours, self.minutes + minutes)
    }
}

impl PartialEq for Clock {
    fn eq(&self, other: &Clock) -> bool {
        self.hours == other.hours && self.minutes == other.minutes
    }
}
