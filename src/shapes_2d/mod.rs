
pub mod area;

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
    pub sides: u16,
    pub radius: f64,
}

impl RegularPolygon {
    pub fn new(sides: u16, radius: f64) -> RegularPolygon {
        RegularPolygon {
            sides,
            radius,
        }
    }
}

///////////////////////////////////////////////////////////////////


