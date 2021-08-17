//Variables hold primitive data or references to data
//Variables are immutable by default (cannot reassign)
//Rust is block-scoped language (varible declare in fn scope of variable only in fn )

use std::mem;

pub fn run(){
    let name = "Josh";
    let mut age = 30; //now we can reassign this varible with "mut" keyword
    println!("My name is {} and I am {}",name,age);

    age = 38; //you can not reassign this age varible.it will give error.
             // if we want to reassign we have to use "mut" before variable.
    println!("My name is {} and I am {}",name,age);

    //define constants
    const ID: i32 = 001; //i32 = integer 32-bit varible name should be all uppercase
    println!("ID: {}",ID);

    //Assign multiple vars
    let (my_name,my_age) = ("Josh",30);
    println!("{} is {}",my_name,my_age);

    //get the size of variable occupied in memory 
    let c = 123456789;
    println!("c = {} and takes up {} bytes",c,mem::size_of_val(&c));

    //usize and isize other new datatypes
    let z: isize = 123;
    let size_of_z: usize = mem::size_of_val(&z);
    println!("z = {}, takes up {} bytes, {}-bit OS",z,size_of_z,size_of_z*8);

    //char
    let d: char = 'x';
    println!("{} is a char, size = {} bytes",d,mem::size_of_val(&d));

    //float f32,f64 standard IEEE754
    let e: f32 = 2.54345;
    println!("e = {}, takes up {} bytes",e,mem::size_of_val(&e));
    let s: f64 = -2.54345123456; //default type - f64
    println!("s = {}, takes up {} bytes",s,mem::size_of_val(&s));

    //boolean
    let g: bool = false;
    println!("g = {}, takes up {} bytes",g,mem::size_of_val(&g));
    
}