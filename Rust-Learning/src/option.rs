
pub fn run() {

    let x = 13.0;
    let y = 0.0;

    //option -> Some(val) | None

    let result = 
        if y != 0.0 { Some(x/y) } else { None };

    match result{
        Some(z) => println!("{}/{} = {}",x,y,z),
        None => println!("cannot divided by zero")    
    }

    if let Some(z) = result{  //let = it will match the result with Some(z) if result is "None" then if statement condition will false.
        println!("result = {}",z)
    }

}