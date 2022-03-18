use chrono::{DateTime, Datelike, Local, Timelike};
pub fn now() -> DateTime<Local> {
    Local::now()
}

pub fn date() -> String {
    now().format("%Y-%m-%d").to_string()
}

pub fn year() -> i32 {
    now().year()
}
pub fn month() -> u32 {
    now().month()
}

pub fn month0() -> u32 {
    now().month0()
}

pub fn day() -> u32 {
    now().day()
}

pub fn time() -> String {
    Local::now().format("%H:%M:%S").to_string()
}

pub fn time_withf() -> String {
    Local::now().format("%H:%M:%S.%f").to_string()
}

pub fn time_with3f() -> String {
    Local::now().format("%H:%M:%S.%3f").to_string()
}

pub fn hour() -> u32 {
    now().hour()
}

pub fn minute() -> u32 {
    now().minute()
}

pub fn second() -> u32 {
    now().second()
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        println!("today is {}", date());
        println!("time is {}", time());
        println!("time_withf is {}", time_withf());
        println!("time_with3f is {}", time_with3f());
        println!("year is {}", year());
        println!("month is {}", month());
        println!("day is {}", day());
        println!("hour is {}", hour());
        println!("minute is {}", minute());
        println!("second is {}", second());
    }
}
