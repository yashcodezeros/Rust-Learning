use std::mem;

struct Point{
    x: f64,
    y: f64
}

fn origin() -> Point{
    Point{x: 0.0,y: 0.0}
}

pub fn run(){
    let p1 = origin(); //stack allocation
    let p2 = Box::new(origin()); //heap allocation

    println!("p1 takes up {} bytes",mem::size_of_val(&p1)); //ans - 16 bytes bcoz 64bits = 8 bytes (x + y) = (8 + 8)
    println!("p2 takes up {} bytes",mem::size_of_val(&p2));
 
    let p3 = *p2; // reference pointer it will get "p2" value.
    println!("{}",p3.x);
}