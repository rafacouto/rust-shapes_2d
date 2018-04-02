
mod shapes_2d;

use shapes_2d::*;
use shapes_2d::area::*;

fn main() {

    let c = Circle::new(1.0);
    let s = Square::new(1.0);
    let r = Rectangle::new(1.0, 1.5);
    let p = RegularPolygon::new(5, 1.0);

    println!("c={:?}, A={:.5}", c, c.area());
    println!("s={:?}, A={:.5}", s, s.area());
    println!("r={:?}, A={:.5}", r, r.area());
    println!("p={:?}, A={:.5}, a={:.5}, s={:.5}",
        p, p.area(), p.apothem(), p.side());

    // hexagon from edge
    let pe = RegularPolygon::from_edge(6, 1.0);
    println!("pe={:?}, s={:.5}", pe, pe.side());

    // octagon from apothem
    let pa = RegularPolygon::from_apothem(8, 1.0);
    println!("pa={:?}, s={:.5}", pa, pa.side());
}

