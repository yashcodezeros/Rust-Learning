//constant - fixed value and u can declare globally and used in everywhere.cannot mutate.
//static  - you can mutate and use everywhere

const MEANING_OF_LIFE:u8 = 42; // no fixed address and provide varible name in all uppercase
static mut Z:i32 = 123; //provide varible name in all uppercase

pub fn run(){
    //you cannot mutate static global varible it is undafe in rust.if u want to mutate then redeclare that static global
    //varible in unsafe block.now u can mutate that.

    unsafe{  
        Z = 456;
        println!("{}",MEANING_OF_LIFE); //you can print.
        println!("{}",Z);
    }

}