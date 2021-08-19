//Structs - Used to create custom data types

//Traditional Struct
// struct Color{
//     red: u8,
//     green: u8,  
//     blue: u8,
// }

//Tuple struct
// struct Color1(u8,u8,u8);

struct Person{
    first_name: String,
    last_name: String
}

impl Person{

    fn new(first: &str,last: &str) -> Person{
        Person{
            first_name: first.to_string(),
            last_name: last.to_string()
        }
    }

    //Get full name
    fn full_name(&self) -> String{
        format!("{} {}",self.first_name,self.last_name)
    }

    //Set last name
    fn set_last_name(&mut self,last: &str){
        self.last_name = last.to_string();
    }

    //Name to tuple
    fn to_tuple(self) -> (String,String){
        (self.first_name,self.last_name)
    }

}

pub fn run(){

    // let mut c = Color{
    //     red: 255,
    //     green: 0,
    //     blue: 0
    // };    

    // c.red = 200;
    // println!("Traditional Struct");
    // println!("Color:{} {} {}",c.red,c.green,c.blue);

    // let mut c1 = Color1(255,0,0);

    // c1.0 = 100;
    // println!("Tuple Struct");
    // println!("Color:{} {} {}",c1.0,c1.1,c1.2);

    let mut p = Person::new("John","Wick");
    //it will print full name : John Wick
    println!("Person: {}",p.full_name());
    p.set_last_name("De Vilears");

    //self fn fullname
    //it will print full name : John De Vilears
    println!("Person: {}",p.full_name());

    //mutate
    p.last_name = String::from("Doe");

    //it will print full name : John Doe
    println!("Person: {} {}",p.first_name,p.last_name);

    //tuple
    println!("Person Tuple: {:?}",p.to_tuple());

}