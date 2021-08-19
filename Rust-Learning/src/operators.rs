fn operators(){

    // Arithmatic
    let mut a= 2+3*5; //+-*
    println!("Afer First Operation a : {}",a); //17
    a = a + 1; // cannot do -- , ++.println!
    println!("After Second Operation a : {}",a); //18
    a -= 2; //can do that +=, -= ,*= ,%=;
    println!("After Third Operation a : {}",a); //16
    println!("Remainder of val {} / {} = {}",a,3,(a%3));

    //power of n (a^N)
    let a_cubed = i32::pow(a,3); //right now a = 16, means a*3 times = 16*16*16 = 4096.
    println!("{} cubed is {}",a,a_cubed);

    //float
    let b = 2.5;
    let b_cubed = f64::powi(b,3);
    let b_to_pi = f64::powf(b,std::f64::consts::PI);
    println!("{} cubed = {}, {}^pi = {}",b,b_cubed,b,b_to_pi);

    //bitwise (only for integers)
    let c = 1 | 2; // "|" -> OR, "&" -> AND, "^" -> XOR, "!" -> NOR
                  // 01 OR 10 = 11 == 3_10
    println!("1 | 2 = {}",c);
    
    //shift 
    let two_to_10 = 1 << 10;
    println!("2^10 = {}",two_to_10);

    //logical operators
    let pi_less_4 = std::f64::consts::PI < 4.0;
    println!("PI < 4 : {}", pi_less_4);
    // "<" ">" "<=" ">=" "=="
    let x = 5;
    let x_is_5 = x == 5;
    println!("x == 5 : {}", x_is_5);

}

pub fn run()
{
    operators();
}