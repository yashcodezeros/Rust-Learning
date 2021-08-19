union IntOrFloat{
    i:i32,
    f:f32
}

fn process_val(iof: IntOrFloat){

    unsafe{
        match iof{
            IntOrFloat { i: 42} => {
                println!("Integer Val");
            }
            IntOrFloat { f } => {
                println!("Float Val: {}",f);
            }
        }
    }
}

pub fn run(){
    let mut iof = IntOrFloat{ i: 123 };
    iof.i = 234;

    let val = unsafe { iof.i }; //we put this in unsafe block bcoz we don't know that it is integer or float.
    println!("iof.i is {}",val);

    process_val(IntOrFloat{i:42});
}