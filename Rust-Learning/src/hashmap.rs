use std::collections::HashMap;

pub fn run(){

    let mut shapes = HashMap::new();
    shapes.insert(String::from("triangle"),3);
    shapes.insert(String::from("square"),4);
    println!("a square has {} sides",shapes["square".into()]);

    //update a old value
    shapes.insert("square".into(),5);

    //find a value if not found then insert
    shapes.entry("circle".into()).or_insert(1);
    {
        let actual = shapes.entry("circle".into()).or_insert(2);
        *actual = 0;
    }

    for (key,val) in &shapes{
        println!("{} : {}",key,val);
    }
}