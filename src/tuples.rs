//Tuples group together values of different types
//Max 12 elements

fn sum_and_product(x:i32, y:i32) -> (i32,i32){
    (x+y, x*y)
}

fn tuples(){
    let x = 5;
    let y = 6;
    let operation = sum_and_product(x, y);

    println!("Operation: {:?}",operation);
    println!("{0} + {1} = {2} and {0} * {1} = {3}",x,y,operation.0,operation.1);

    //destructuring
    let (a,b) = operation; //it will both result of a=sum and b=product
    println!("After Destructuring a = {} and b = {}",a,b);

    let operation2 = sum_and_product(11,23);
    let combined = (operation,operation2);
    println!("Both tuples{:?}",combined);

    //get the last element from tuples
    println!("Last elmnt = {}",(combined.1).1);

    //can do multiple tuples destructuring
    let ((c,d),(e,f)) = combined;
    println!("After Destructuring c = {} and d = {} and e = {} and f = {}",c,d,e,f);
}

pub fn run(){

    let person: (&str,&str,i8) = ("Josh","Mass",30);

    println!("{name} is from {location} and {name} is {age}",name = person.0,location = person.1,age = person.2);

    tuples();

}