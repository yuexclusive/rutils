#![allow(dead_code)]

trait Shape {
    fn draw(&self) -> String;
}

trait ShapeFactory {
    fn new(&self) -> Box<dyn Shape>;
}

fn get_shape<T>(factory: T) -> Box<dyn Shape>
where
    T: ShapeFactory,
{
    return factory.new();
}

struct Rectangle {}

impl Shape for Rectangle {
    fn draw(&self) -> String {
        "draw a rectangle!".to_string()
    }
}

struct Circle {}

impl Shape for Circle {
    fn draw(&self) -> String {
        "draw a circle!".to_string()
    }
}

struct RectangleFactory {}
struct CircleFactory {}

impl ShapeFactory for CircleFactory {
    fn new(&self) -> Box<dyn Shape> {
        Box::new(Circle {})
    }
}

impl ShapeFactory for RectangleFactory {
    fn new(&self) -> Box<dyn Shape> {
        Box::new(Rectangle {})
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_circle() {
        let s = get_shape(CircleFactory {});
        assert_eq!(s.draw(), "draw a circle!")
    }

    #[test]
    fn test_rectangle() {
        let s = get_shape(RectangleFactory {});
        assert_eq!(s.draw(), "draw a rectangle!")
    }
}
