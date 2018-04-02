
////////////////////////////////////////////////////////////////

pub trait Area2d {
    fn area(&self) -> f64;
}

////////////////////////////////////////////////////////////////

const PI: f64 = 3.14159265;

////////////////////////////////////////////////////////////////

use shapes_2d::*;

impl Area2d for Circle {
    fn area(&self) -> f64 {
       self.radius * self.radius * PI
    }
}


impl Area2d for Square {
    fn area(&self) -> f64 {
       self.side * self.side
    }
}


impl Area2d for Rectangle {
    fn area(&self) -> f64 {
       self.side1 * self.side2
    }
}

////////////////////////////////////////////////////////////////


