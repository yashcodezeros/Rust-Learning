struct Creature{
    name: String
}

impl Creature{
    fn new(name: &str) -> Creature{
        println!("{} enters the game",name);
        Creature { name: name.into() }
    }
}

impl Drop for Creature{
    fn drop(&mut self){
        println!("{} is dead",self.name);
    }
}

pub fn run(){

    let goblin = Creature::new("Jeff");
    println!("Game proceed!");
    drop(goblin); //now after that u can;t use goblin

}