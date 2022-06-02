use serde::{Deserialize, Serialize};
use std::fs;

use lazy_static::lazy_static;

lazy_static! {
    pub static ref CONFIG: Config = {
        let config_path = "./config.toml";
        let str = fs::read_to_string(config_path).unwrap();
        let config: Config = toml::from_str(&str).unwrap();
        config
    };
}

#[derive(Deserialize, Serialize)]
pub struct Redis {
    pub address: String,
    pub password: String,
}

#[derive(Deserialize, Serialize)]
pub struct Mysql {
    pub address: String,
}

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub name: String,
    pub mysql: Mysql,
    pub redis: Redis,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1() {
        let name = &CONFIG.name;
        assert_eq!("rocket_demo", name)
    }
}
