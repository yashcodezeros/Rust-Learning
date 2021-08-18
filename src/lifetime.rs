struct Person {
    name: String,
}

struct Company<'z> {
    name: String,
    ceo: &'z Person, //lifetime as " 'static "
}

impl Person {
    // fn new(name: &str) -> Person {
    //     Person {
    //         name: name.to_string(),
    //     }
    // }

    //fn get_ref_name(&self) => String{
    fn get_ref_name<'a>(&'a self) -> &'a String {
        //lifetime isslutsion
        &self.name
    }

    // fn name(&self) -> String {
    //     format!("{}", self.name)
    // }
}

// impl Company {
//     fn new(name: &str, ceo: &str) -> Company {
//         Company {
//             name: name.to_string(),
//             ceo: ceo.to_string(),
//         }
//     }

//     fn name(&self) -> String {
//         format!("{}", self.name)
//     }
// }
pub fn run() {
    let z: &String;
    {
        let boss = Person{ name: String::from("Elon musk")};
        z = boss.get_ref_name();
        // let tesla = Company {
        //     name: String::from("Tesla"),
        //     ceo: &boss,
        // };

        println!("{} is the CEO", z,);
    }
}
