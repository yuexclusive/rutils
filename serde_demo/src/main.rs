// #![cfg(feature = "toml")]
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct Config {
    ip: String,
    port: Option<u16>,
    keys: Keys,
}

#[derive(Deserialize, Serialize)]
struct Keys {
    github: String,
    travis: Option<String>,
}

fn main() {
    test()
}

fn test() {
    let str = r#"
    ip = '127.0.0.1'
    port = 8080
    [keys]
    github = 'xxxxxxxxxxxxxxxxx'
    travis = 'yyyyyyyyyyyyyyyyy'
"#;
    let config: Config = toml::from_slice(str.as_bytes()).unwrap();

    let str = toml::to_string(&config).unwrap();

    let config: Config = toml::from_str(&str).unwrap();

    println!("toml: \n{}", str);

    assert_eq!(config.ip, "127.0.0.1");
    assert_eq!(config.port, Some(8080));
    assert_eq!(config.keys.github, "xxxxxxxxxxxxxxxxx");
    assert_eq!(config.keys.travis.as_ref().unwrap(), "yyyyyyyyyyyyyyyyy");

    let str = serde_json::to_string(&config).unwrap();

    println!("json: \n{}", str);

    let config: Config = serde_json::from_str(&str).unwrap();

    assert_eq!(config.ip, "127.0.0.1");
    assert_eq!(config.port, Some(8080));
    assert_eq!(config.keys.github, "xxxxxxxxxxxxxxxxx");
    assert_eq!(config.keys.travis.as_ref().unwrap(), "yyyyyyyyyyyyyyyyy");

    let str = serde_yaml::to_string(&config).unwrap();

    println!("yaml: \n{}", str);

    let config: Config = serde_yaml::from_str(&str).unwrap();

    assert_eq!(config.ip, "127.0.0.1");
    assert_eq!(config.port, Some(8080));
    assert_eq!(config.keys.github, "xxxxxxxxxxxxxxxxx");
    assert_eq!(config.keys.travis.as_ref().unwrap(), "yyyyyyyyyyyyyyyyy");
}
