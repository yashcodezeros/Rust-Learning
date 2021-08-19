//Loops - Used to iterate untill condition is met
fn while_loop(){
let mut z = 1;
    while z < 1000{
        z *= 2;

        if z == 32{
            continue; //it will stop loop and start execution from top
        }

        // if z == 32{  
        //     break; //it will break loop and stop execution
        // }

        println!("z = {}",z)
    }

}

fn for_loop(){

    for (position,y) in (0..11).enumerate(){
        println!("{} : {}",position,y);
    }

}

pub fn run(){

    //infinite loop
    println!("!Infinte Loop START!");
    let mut count = 0;
    loop{
        count += 1;
        println!("Number: {}",count);

        if count == 20{
            break;
        }
    }
    println!("!Infinte Loop END!");

    // while loop(FizzBuzz)
    println!("!While Loop START!");
    let mut value = 0;
    while value <= 100{
        if value % 15 == 0{
            println!("fizzbuzz");
        }else if value % 3 == 0{
            println!("fizz");
        }else if value % 5 == 0{
            println!("buzz");
        }else {
            println!("Numbers: {}",value);
        }

        //Incremet
        value += 1;
    }
    println!("!While Loop END!");

    //for range loop
    println!("!For range Loop START!");
    for x in 0..100{
        if x % 15 == 0{
            println!("fizzbuzz");
        }else if x % 3 == 0{
            println!("fizz");
        }else if x % 5 == 0{
            println!("buzz");
        } else {
            println!("Numbers: {}",x);
        }
    }
    println!("!For range Loop END!");

    while_loop();
    for_loop();
}