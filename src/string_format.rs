#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_must_use)]

use std::thread;
use std::time;

pub fn run(){

    let name = "Josh";
    let age = 20;
    let que = format!("Hello, {} are u {} years old?",name,age);
    println!("{}",que);

    //with postions arguments
    let name = "Adam";
    let age = 18;
    let activity = "Football";
    let position_arg = format!("{0} is {1} years old and {0} likes to play {2}.",name,age,activity);
    println!("{}",position_arg);

    //named arguments
    let named_arg = format!("{name} is {age} years old and {name} likes to play {activity}.",name="Riya",age=21,activity="Guitar");
    println!("{}",named_arg);

    //mixed arguments
    let mixed_arg = format!("{} is {1} years old and {0} likes to play {activity}.","Aman",11,activity="Cricket");
    println!("{}",mixed_arg);

}