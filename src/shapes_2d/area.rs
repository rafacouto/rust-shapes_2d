
////////////////////////////////////////////////////////////////

pub trait Area2d {
    fn area(&self) -> f64;
}

////////////////////////////////////////////////////////////////

use std::f64::consts::PI;

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


impl Area2d for RegularPolygon {
    fn area(&self) -> f64 {
        let s = 2.0 * PI / self.edges as f64;
        0.5 * self.edges as f64 * self.radius * self.radius * s.sin()
    }
}

////////////////////////////////////////////////////////////////


