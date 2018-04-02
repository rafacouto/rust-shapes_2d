
pub mod area;

use std::f64::consts::PI;

///////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub struct Circle {
    pub radius: f64,
}

impl Circle {
    pub fn new(radius: f64) -> Circle {
        Circle {
            radius
        }
    }
}


///////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub struct Square {
    pub side: f64,
}

impl Square {
    pub fn new(side: f64) -> Square {
        Square {
            side
        }
    }
}

///////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub struct Rectangle {
    pub side1: f64,
    pub side2: f64,
}

impl Rectangle {
    pub fn new(side1: f64, side2: f64) -> Rectangle {
        Rectangle {
            side1,
            side2,
        }
    }
}

///////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub struct RegularPolygon {
    pub edges: u16,
    pub radius: f64,
}


impl RegularPolygon {
    pub fn new(edges: u16, radius: f64) -> RegularPolygon {
        RegularPolygon {
            edges,
            radius,
        }
    }

    pub fn apothem(&self) -> f64 {
        self.radius * (PI / self.edges as f64).cos()
    }

    pub fn side(&self) -> f64 {
        self.radius * 2.0 * (PI / self.edges as f64).sin()
    }

    pub fn from_edge(edges: u16, length: f64) -> RegularPolygon {
        let r = length / (2.0 * (PI / edges as f64).sin());
        RegularPolygon::new(edges, r)
    }

    pub fn from_apothem(edges: u16, length: f64) -> RegularPolygon {
        let r = length / (PI / edges as f64).cos();
        RegularPolygon::new(edges, r)
    }
}

///////////////////////////////////////////////////////////////////


