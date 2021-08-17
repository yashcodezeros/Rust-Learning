use std::mem;

fn double_value(v: i32) -> i32{
    v*2
}

pub fn run(){
    let mut x:i32 = 3;
    x = double_value(x);
    x=42;
}