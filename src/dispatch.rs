trait Printable {
    fn format(&self) -> String;
}

impl Printable for i32 {
    fn format(&self) -> String {
        format!("i32: {}", *self)
    }
}

impl Printable for String {
    fn format(&self) -> String {
        format!("String: {}", *self)
    }
}

//static dispatch
fn print_it_static<T: Printable>(z: T) { 
    println!("{}",z.format());
} //monomorphisation

//dynamic dispatach
fn print_it_dynamic(z: &Printable){
    println!("{}",z.format());
}

//Dynamic Dispatch

struct Circle { radius: f64 }
struct Square { side: f64 }

trait Shape{
    fn area(&self) -> f64;
}

impl Shape for Square{
    fn area(&self) -> f64{
        self.side * self.side
    }
}

impl Shape for Circle{
    fn area(&self) -> f64{
        self.radius * self.radius * std::f64::consts::PI
    }
}

pub fn run() {
    let a = 322;
    let b = "Hello".to_string();

    // println!("{}", a.format());
    // println!("{}", b.format());

    // print_it_static(a); //Static Dispatch
    // print_it_static(b); //Static Dispatch

    print_it_dynamic(&a); //Dynamic Dispatch
    print_it_dynamic(&b); //Dynamic Dispatch

    let shapes:[&Shape; 4] = [
        &Circle{radius: 1.0},
        &Square{side:3.0},
        &Circle{radius:4.0},
        &Square{side:2.0}
    ];
    for (i, shape) in shapes.iter().enumerate(){
        println!("Shape {}# has area {}",i,shape.area());
    }

}
