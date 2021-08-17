// Conditionals - Used to check the condition of something and act on the result

fn if_statements(){

    let temp = 23;

    if temp >= 30{
        println!("we can't go outside its really hot outside!");
    }else if temp <= 10{
        println!("we can't go outside its really cold outside!!");
    }else{
        println!("we can go outside bcoz now temperature is normal!!");
    }

    let day = if temp > 20 {"Sunny"} else {"Cloudy"};
    println!("Today is {}",day);

    //inside println!
    println!("it is {}",
        if temp > 20 {"Hot"} else if temp < 10 {"Cold"} else {"OK"}
    );

    //inside println nested if
     println!("it is {}",
        if temp > 20{
            if temp >= 30 {"Very hot"} else {"Hot"}
        }else if temp < 10 {"Very cold"} else {"OK"}
    );
}

pub fn run(){
    let age: u8 = 21;
    let check_id: bool = true;
    let knows_person_of_age = true;

    if age >= 21 && check_id || knows_person_of_age{
        println!("Bartender: What would you like to drink?");
    }else if age < 21 && check_id{
        println!("Bartender: Sorry, you have to leave");
    }else{
        println!("Bartender: I'll need to check your ID");
    }

    //shorthand
    let is_of_age = if age >= 21 { true } else { false };
    println!("{}",is_of_age);

    if_statements();
    
}