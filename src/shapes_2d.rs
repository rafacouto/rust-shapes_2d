
///////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub struct Circle {
    radius: f64,
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
    side: f64,
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
    side1: f64,
    side2: f64,
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
    sides: u16,
    radius: f64,
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


