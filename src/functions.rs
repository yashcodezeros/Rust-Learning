//Functions - Used to store blocks of code for re-use
fn say_hello() {
    println!("Hello Rust!!");
}

//Higher order functions
//functions that take functions e.g - f(g) { let x = g() };
//function that return functions (generators)

//sum of all even squares < 500
fn is_even(x:u32) -> bool{
    x % 2 == 0
}

fn greater_than(limit: u32) -> impl Fn(u32) -> bool{ //function that return function
   move |y| y > limit
}
fn higher_order_functions(){

    let limit = 500;
    let mut sum = 0;

    // let above_limit = |y| y > limit; //for checking limit.
    let above_limit = greater_than(limit);
    for i in 0..{
        let isq = i*i;
        // if isq > limit{
        if above_limit(isq){
            break;
        } else if is_even(isq){
            sum += isq
        }
    }

    println!("loop sum: {}",sum);

    //function that take functions
    let sum2 = (0..).map(|x| x*x).take_while(|&x| x < limit).filter(|x:&u32| is_even(*x)).fold(0, | sum, x | sum + x );
    println!("loop sum 2: {}",sum2);

}

pub fn run() {
    //call simple fn
    greeting("Hello", "Josh");

    let sh = say_hello; //variable refer to a function.
    sh(); // now u can call function like this.

    //bind function values to variables
    let get_sum = add(10, 20);
    println!("Sum: {}", get_sum);

    //Closure
    let num3: i32 = 10;
    let add_nums = |num1: i32, num2: i32| num1 + num2 + num3;
    println!("Closure sum: {}", add_nums(4, 7));

    let mut two = 2;
    let add_numbers = |x| //not setting the datatype
    {
        let mut z = x;
        z += two;
        z
    };
    println!("{} + 2 = {}", 3, add_numbers(3));
    let borrow_two = &mut two;

    //if u want to update value after decaration and operation
    let new_nums = |x:&mut i32| *x += 3;
    let mut f = 12;
    new_nums(&mut f);
    println!("f: {}",f);

    higher_order_functions();

}

//simple function
fn greeting(greet: &str, name: &str) {
    println!("{} {}, nice to meet you.", greet, name);
}

//function that return sum of two values
fn add(num1: i32, num2: i32) -> i32 {
    num1 + num2
}
