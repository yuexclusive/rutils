pub use serde::{Deserialize, Serialize};
use std::error::Error;

pub fn to_string<T>(val: &T) -> Result<String, Box<dyn Error>>
where
    T: ?Sized + Serialize,
{
    let res = serde_json::to_string(val)?;
    Ok(res)
}

pub fn from_str<'a, T>(s: &'a str) -> Result<T, Box<dyn Error>>
where
    T: Deserialize<'a>,
{
    let res = serde_json::from_str(s)?;
    Ok(res)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    struct Point {
        x: i32,
        y: i32,
    }

    #[test]
    fn it_work() {
        let point = Point { x: 1, y: 2 };

        // Convert the Point to a JSON string.
        let serialized = to_string(&point).unwrap();

        // Prints serialized = {"x":1,"y":2}
        println!("serialized = {}", serialized);

        // Convert the JSON string back to a Point.
        let deserialized: Point = from_str(&serialized).unwrap();

        // Prints deserialized = Point { x: 1, y: 2 }
        assert_eq!(point, deserialized);
    }
}
