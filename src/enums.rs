//Enums are types which have a few definite values

enum Movement {
    //Variants
    Up,
    Down,
    Left,
    Right,
}

enum Color {
    Red,
    Green,
    Blue,
    RgbColor(u8,u8,u8), //tuple
    Cmyk{ cyan:u8, magenta:u8, yellow:u8, black:u8 } //struct
}

fn move_avtar(m: Movement) {
    //Perform action depending on info

    match m {
        Movement::Up => println!("Avatar moving up"),
        Movement::Down => println!("Avatar moving down"),
        Movement::Left => println!("Avatar moving left"),
        Movement::Right => println!("Avatar moving right"),
    }
}

fn colors(){
    // let c:Color = Color::RgbColor(10,0,0);  //tuple
    let c:Color = Color::Cmyk{cyan:0, magenta:128, yellow:0, black:255 }; //struct
    match c{
        Color::Red => println!("Color Red!!"),
        Color::Green => println!("Color Green!!"),
        Color::Blue => println!("Color Blue!!"),
        Color::Cmyk{cyan:_, magenta:_, yellow:_, black: 255} => println!("CMYK Color Black!!"),
        Color::RgbColor(0,0,0) => println!("Color Black!!"),
        Color::RgbColor(r,g,b) => println!("rgb({},{},{})",r,g,b),
        _ => println!("Other Colors!!"),
    }
}

pub fn run() {
    let avatar1 = Movement::Left;
    let avatar2 = Movement::Up;
    let avatar3 = Movement::Right;
    let avatar4 = Movement::Down;

    move_avtar(avatar1);
    move_avtar(avatar2);
    move_avtar(avatar3);
    move_avtar(avatar4);

    colors();
}
