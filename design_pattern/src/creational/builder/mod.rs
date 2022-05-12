#![allow(dead_code)]
#![allow(unused)]

#[derive(Clone)]
struct Product {
    vec: Vec<String>,
}

impl Product {
    fn new() -> Self {
        Product { vec: Vec::new() }
    }
    fn show(&self) -> String {
        self.vec.join("\n")
    }
}

trait Builder {
    fn step_1(&mut self);
    fn step_2(&mut self);
    fn step_3(&mut self);
    fn get(&self) -> Product;
}

struct BuilderA {
    product: Product,
}
struct BuilderB {
    product: Product,
}

impl BuilderA {
    fn new() -> Self {
        BuilderA {
            product: Product::new(),
        }
    }
}

impl BuilderB {
    fn new() -> Self {
        BuilderB {
            product: Product::new(),
        }
    }
}

impl Builder for BuilderA {
    fn step_1(&mut self) {
        self.product.vec.push(String::from("builder a 1"))
    }

    fn step_2(&mut self) {
        self.product.vec.push(String::from("builder a 2"))
    }

    fn step_3(&mut self) {
        self.product.vec.push(String::from("builder a 3"))
    }

    fn get(&self) -> Product {
        self.product.clone()
    }
}

impl Builder for BuilderB {
    fn step_1(&mut self) {
        self.product.vec.push(String::from("builder b 1"))
    }

    fn step_2(&mut self) {
        self.product.vec.push(String::from("builder b 2"))
    }

    fn step_3(&mut self) {
        self.product.vec.push(String::from("builder b 3"))
    }

    fn get(&self) -> Product {
        self.product.clone()
    }
}

struct Director<T>
where
    T: Builder,
{
    builder: T,
}

impl<T> Director<T>
where
    T: Builder,
{
    fn new(b: T) -> Self {
        Director { builder: b }
    }

    fn construct(&mut self) -> Product {
        self.builder.step_1();
        self.builder.step_2();
        self.builder.step_3();
        self.builder.get()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_builder_a() {
        let mut director = Director::new(BuilderA::new());
        let product = director.construct();
        let want = r#"builder a 1
builder a 2
builder a 3"#;

        assert_eq!(product.show(),want)
    }

    #[test]
    fn test_builder_b() {
        let mut director = Director::new(BuilderB::new());
        let product = director.construct();
        let want = r#"builder b 1
builder b 2
builder b 3"#;

        assert_eq!(product.show(),want)
    }
}
