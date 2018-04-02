
mod shapes_2d;

use shapes_2d::*;
use shapes_2d::area::*;

fn main() {

    let c = Circle::new(1.0);
    let s = Square::new(1.0);
    let r = Rectangle::new(1.0, 1.5);
    let p = RegularPolygon::new(5, 1.0);

    println!("c={:?}, A={}", c, c.area());
    println!("s={:?}, A={}", s, s.area());
    println!("r={:?}, A={}", r, r.area());
    println!("p={:?}", p);
}

