use chrono::{DateTime, Datelike, Duration, Local, Timelike};

pub struct Time {
    pub now: DateTime<Local>,
}

pub fn now() -> Time {
    Time::new(Local::now())
}

pub fn nanoseconds(nanoseconds: i64) -> Duration {
    Duration::nanoseconds(nanoseconds)
}

pub fn microseconds(microseconds: i64) -> Duration {
    Duration::microseconds(microseconds)
}

pub fn milliseconds(milliseconds: i64) -> Duration {
    Duration::milliseconds(milliseconds)
}

pub fn second(seconds: i64) -> Duration {
    Duration::seconds(seconds)
}

pub fn minutes(minutes: i64) -> Duration {
    Duration::minutes(minutes)
}

pub fn hours(hours: i64) -> Duration {
    Duration::hours(hours)
}

pub fn days(days: i64) -> Duration {
    Duration::days(days)
}

impl Time {
    fn new(dt: DateTime<Local>) -> Time {
        Self { now: dt }
    }

    pub fn date(&self) -> String {
        format!("test for {}", self.now.format("%Y-%m-%d").to_string())
    }

    pub fn year(&self) -> i32 {
        self.now.year()
    }
    pub fn month(&self) -> u32 {
        self.now.month()
    }

    pub fn month0(&self) -> u32 {
        self.now.month0()
    }

    pub fn day(&self) -> u32 {
        self.now.day()
    }

    pub fn day0(&self) -> u32 {
        self.now.day0()
    }

    pub fn time(&self) -> String {
        self.now.format("%H:%M:%S").to_string()
    }

    pub fn time_withf(&self) -> String {
        self.now.format("%H:%M:%S.%f").to_string()
    }

    pub fn time_with3f(&self) -> String {
        self.now.format("%H:%M:%S.%3f").to_string()
    }

    pub fn hour(&self) -> u32 {
        self.now.hour()
    }

    pub fn minute(&self) -> u32 {
        self.now.minute()
    }

    pub fn second(&self) -> u32 {
        self.now.second()
    }

    pub fn weekday(&self) -> chrono::Weekday {
        self.now.weekday()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        println!("today is {}", now().date());
        println!("time is {}", now().time());
        println!("time_withf is {}", now().time_withf());
        println!("time_with3f is {}", now().time_with3f());
        println!("year is {}", now().year());
        println!("month is {}", now().month());
        println!("day is {}", now().day());
        println!("hour is {}", now().hour());
        println!("minute is {}", now().minute());
        println!("second is {}", now().second());
        println!("weekday is {}", now().weekday());
        let a = std::time::SystemTime::now();
        std::thread::sleep(std::time::Duration::from_secs(1));
        let res = std::time::SystemTime::now()
            .duration_since(a)
            .unwrap()
            .as_nanos();

        let x = std::time::SystemTime::now();
        println!("{:?}\n{:?}", res, x)
    }
}
