//Functions - Used to store blocks of code for re-use

pub fn run(){
    //call simple fn
    greeting("Hello", "Josh");

    //bind function values to variables
    let get_sum =  add(10,20);
    println!("Sum: {}",get_sum);

    //Closure
    let num3: i32 = 10;
    let add_nums = |num1: i32,num2: i32| num1 + num2 + num3;
    println!("Closure sum: {}",add_nums(4,7));
}

//simple function
fn greeting(greet: &str,name: &str){
    println!("{} {}, nice to meet you.",greet,name);
}

//function that return sum of two values
fn add(num1: i32,num2: i32) -> i32{
    num1 + num2
}