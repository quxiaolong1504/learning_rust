//! # The first line
//! The second line
/// Adds one to the number given.
///
/// # Examples
///
///
use std::f64::consts::PI;

fn main() {
    println!("Hello, world!");
}

trait HasArea {
    fn area(&self) -> f64;
}

struct Cricel {
    x: f64,
    y: f64,
    radius: f64
}

impl HasArea for Cricel {
    fn area(&self) -> f64{
        PI * self.radius * self.radius
    }
}

struct Square {
    x: f64,
    y: f64,
    size: f64
}

impl HasArea for Square{
    fn area(&self) -> f64{
        self.size * self.size
    }
}

fn print_area<T: HasArea>(shape: T){
    println!("This shape has an area of {}", shape.area());
}