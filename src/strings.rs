//Primitive str = immutable fixed-length string somewhere in memory
//String = Growable, heap-allocated data structure - Use when you need to modify or own string data 

fn strings(){

    //utf-8
    let b:&'static str = "Hello John!"; //&str = string slice
    //a = "abc"; //cannot do that
    //let g = a[0]; //cannot do that

    for d in b.chars().rev(){
        println!("{}",d);
    }

    if let Some(first_char) = b.chars().nth(0){
        println!("First letter: {}",first_char);
    }

    //heap
    //string
    let mut letters = String::new();
    let mut a = 'a' as u8;
    while a <= ('z' as u8){
        letters.push(a as char);
        letters.push_str(",");
        a += 1;
    }
    println!("{}",letters);

    //&str <> String
    let u:&str = &letters;

    //concatenation
    //String + str
    //let z = letters + & letters;

    let mut abc = "hello Josh".to_string();
    abc.remove(0);
    abc.push_str("!!!");
    println!("{}",abc.replace("ello","Goodbye"));

}

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

    strings();
}