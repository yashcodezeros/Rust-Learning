
fn scope_shadowing(){

    let a = 123;    
    let a = 789; // you can redeclare same and it accept latest value.
    {
        let b  = 456;
        let a = 12345; //outside declared "a" and inside declared "a" both are diffrent.
                      // if we doesn't declare "a" inside in whole scope the outside "a" is referenced.
                     // this is called varible shadowing.
        println!("inside a = {}",a);
        println!("inside b = {}",b);
    }

    println!("outside a = {}",a);
    // println!("outside b = {}",b); //it will give error bcoz "b" doesn't exist in this scope.

}

pub fn run(){

    // println!("{}",a);  //it will give error because rust is block scoped laguage.
                         //you cannot access varible outside its block scope.

    scope_shadowing();
}   