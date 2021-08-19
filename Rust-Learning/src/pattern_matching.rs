fn how_many(x:i32) -> &'static str{
    match x{
        0 => "no",
        1 | 2 => "One or Two ",
        12 => "Dozen",
        _ if (x % 2 == 0) => "even",
        _ => "a few"
    }   
}

fn pattern_matching(){
    for x in 0..13{
        println!("{}: I have {} oranges",x,how_many(x));
    }

    let point = (0,7);

    match point{
        (0,0) => println!("origin"),
        (0,y) => println!("x axis, y = {}",y),
        (x,0) => println!("x = {},y axis",x),
        (x,y) => println!("x = {}, y = {}",x,y),
    }
}

pub fn run(){
    pattern_matching();
}