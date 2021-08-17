//Primitive str = immutable fixed-length string somewhere in memory
//String = Growable, heap-allocated data structure - Use when you need to modify or own string data 

pub fn run(){

    let mut hello = String::from("Hello ");

    hello.push('W'); //push only one character
    hello.push_str("orld!"); //push whole string

    //Capacity in bytes
    println!("Capacity: {}", hello.capacity());

    //check string is empty
    println!("Isempty: {}", hello.is_empty());

    //contains
    println!("Contains 'World': {}", hello.contains("World"));

    //replace
    println!("Replace 'World': {}", hello.replace("World","Bhavik"));

    //loop through string by whitespcae 
    for word in hello.split_whitespace(){
        println!("Loop: {}",word);
    }

    //cretae string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');
    println!("{}",s);

    //Assertion testing
    assert_eq!(2,s.len()); //(left == right) ex: 2 == 2(s.len())
    assert_eq!(10,s.capacity());

    //get length
    println!("Length: {}",hello.len());
    println!("{}",hello);
}