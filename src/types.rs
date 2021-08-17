/*
Primitive types--
Integers: u8,i8,u16,i16,u32,i32,u64,i64,u128,i128(numbers of bits taken in memory),(u = unsigned(no-negative values) integers,i = normal integers)
Floats: f32,f64,
Boolean: (bool),
Characters: (char),
Tuples
Arrays
*/

//Rust is statically typed language, which means that it must know the types of all varibles at the compile time.
//However, the compiler can usually infer what type we want to use based on the value and how to use it.
pub fn run(){

    //By defult it is "i32".
    let x = 10;

    //By defult it is "f64".
    let y = 4.5;

    //Add explicit type
    let z: i64 = 3434343434123;

    //find max size
    println!("max i32: {}", std::i32::MAX);
    println!("max i64: {}", std::i64::MAX);

    //Boolean
    let is_active: bool = true;

    //Get boolean from experssion
    let is_greater: bool = 10 < 5;

    //Character
    let a = 'h'; //it only allow one character('a') not string ("hello").
    let emoji = '\u{1F600}'; //use to display emoji in character type.

    println!("{:?}",(x,y,z,is_active,is_greater,a,emoji));

}