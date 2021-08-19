pub fn run(){
    //match keyword is use to match all the cases if user not provide all cases it will give u compile error.println!
    //it will find match cases based on datatype automatically user does not specify the datatype.

    let country_code = 1445;
    let country = match country_code{
        44 => "UK",
        46 => "Sweden",
        7 => "Russia",
        1..=1000 => "Unknown",
        _ => "Invalid" //it supports if all above cases not valid then it will be apply.
    };

    println!("The country with code {} is {}",country_code,country);

    let x = false;

    let s = match x{
        true => "yes",
        false => "no"
    };

    println!("x is {}",x);
}