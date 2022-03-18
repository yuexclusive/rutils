use json;

#[derive(json::Serialize, json::Deserialize, Debug)]
struct Person {
    pub name: String,
    pub age: i32,
}

fn main() {
    let str = json::to_string(&Person {
        name: "haha".to_string(),
        age: 11,
    })
    .unwrap();

    let p: Person = json::from_str(&str).unwrap();

    println!("{:?}", p)
}
