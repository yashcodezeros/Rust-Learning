pub fn run(){
    //print to console
    println!("Hello from print.rs");

    //Basic formatting
    println!("Numbar: {}",1);
    println!("{} is from {}","Josh","Amsterdam");

    //Positional Arguments
    println!("{0} is from {1} and {0} likes {2}","Josh","Amsterdam","Football");

    //Named Arguments
    println!("{name} likes to play {activity}",name="Johny",activity="cricket");

    //Placeholder traits
    println!("Binary: {:b}, Hex: {:x}, Octal: {:o}",10,10,10);

    //Placeholder for debug trait
    println!("{:?}",(12,true,"hello"));

    //Basic math
    println!("10 + 10 = {}",10 + 10);
}