mod shapes_2d;

use shapes_2d::*;

fn main() {

    let c = Circle::new(1.0);
    let s = Square::new(1.0);
    let r = Rectangle::new(1.0, 1.5);
    let p = RegularPolygon::new(5, 1.0);

    println!("c: {:?}", c);
    println!("s: {:?}", s);
    println!("r: {:?}", r);
    println!("p: {:?}", p);
}

