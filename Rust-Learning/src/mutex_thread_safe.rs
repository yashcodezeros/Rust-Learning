use std::sync::{Mutex,Arc};
use std::thread;

struct Person {
    name: Arc<String>,
    state: Arc<Mutex<String>>
}

impl Person{
    fn new(name: Arc<String>, state: Arc<Mutex<String>>) -> Person{
        Person { name: name, state: state }
    }

    fn greet(&self){
        let mut state = self.state.lock().unwrap();
        state.clear();
        state.push_str("excited!");
        println!("Hi, my name is {} and i am {}",self.name,state.as_str());
    }
}


fn mutex_demo(){
    let name = Arc::new("Scout".to_string());
    let state = Arc::new(Mutex::new("Bored".to_string()));
    let person = Person::new(name.clone(),state.clone());
    let t = thread::spawn(move || { person.greet();} );
    println!("Name = {} and state = {}",name,state.lock().unwrap().as_str());
    t.join().unwrap();
}

pub fn run(){
    mutex_demo();
}