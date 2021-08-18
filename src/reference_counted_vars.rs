use std::rc::Rc;
use std::sync::Arc;
use std::thread;


struct Person {
    name: Rc<String>
}

struct Person2 {
    name: Arc<String>
}

impl Person{
    fn new(name: Rc<String>) -> Person{
        Person { name: name }
    }

    fn greet(&self){
        println!("Hi, my name is {}",self.name);
    }
}

impl Person2{
    fn new(name: Arc<String>) -> Person2{
        Person2 { name: name }
    }

    fn greet(&self){
        println!("Hi, my name is {}",self.name);
    }
}

fn rc_demo(){
    let name = Rc::new("Johnathan".to_string());
    println!("Name = {}, name has {} strong pointers",name,Rc::strong_count(&name));
    {
        let person = Person::new(name.clone()); // we borrowed the name value here so now we cannot use "name" variable.
                                   // if we have to use we have to implement "rc" trait.
        println!("Name = {}, name has {} strong pointers",name,Rc::strong_count(&name));
        person.greet();
    }
    println!("Name = {}",name);
    println!("Name = {}, name has {} strong pointers",name,Rc::strong_count(&name));
}

fn arc_demo(){
    let name = Arc::new("Scout".to_string());
    let person = Person2::new(name.clone());
    let t = thread::spawn(move || { person.greet();} );
    println!("Name = {}",name);
    t.join().unwrap();
}

pub fn run(){
    rc_demo();
    arc_demo();
}