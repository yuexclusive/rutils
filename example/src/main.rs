#[derive(Debug)]
struct Person {
    age: i32,
}

impl Drop for Person {
    fn drop(&mut self) {
        println!("{}", self.age);
        println!("{}", "person droped")
    }
}

fn main() {
    let a = 1;
    println!("{}", a);
}
