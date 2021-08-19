trait Animal {
    fn create(name: &'static str) -> Self;
    fn name(&self) -> &'static str;
    fn talk(&self) {
        println!("{} cannot talk", self.name());
    }
}

trait Summable <T>{
    fn sum(&self) -> T;
}

struct Human {
    name: &'static str,
}

struct Cat {
    name: &'static str,
}

impl Animal for Human {
    fn create(name: &'static str) -> Human {
        Human { name: name }
    }
    fn name(&self) -> &'static str {
        self.name
    }
    fn talk(&self) {
        println!("{} says Hello!!", self.name());
    }
}

impl Animal for Cat {
    fn create(name: &'static str) -> Cat {
        Cat { name: name }
    }
    fn name(&self) -> &'static str {
        self.name
    }
    fn talk(&self) {
        println!("{} says Meow!!", self.name());
    }
}

impl Summable<i32> for Vec<i32>{
    fn sum(&self) -> i32{
        let mut result:i32 = 0;
        for x in self { result += *x; }
        return result;
    }
}

fn traits() {
    // let h = Human { name: "Josh" };
    // let h = Human::create("John"); //create fn used
    let h:Human = Animal::create("John"); //animal trait create fn used
    h.talk();

    // let c = Cat::create("Adriana"); //create fn used
    let c:Cat = Animal::create("Adriana"); //animal trait create fn used
    // let c = Cat { name: "Misty" };
    c.talk();

    let a = vec![1,2,3];
    println!("Sum is {}",a.sum());
}
pub fn run() {
    traits();
}
