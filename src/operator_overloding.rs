#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_must_use)]
#![allow(unused_mut)]

use std::ops::Add;

#[derive(Debug)]
struct ComplexNums<T> {
    re: T,
    im: T,
}

impl<T> ComplexNums<T> {
    fn new(re: T, im: T) -> ComplexNums<T> {
        ComplexNums::<T> { re, im }
    }
}

impl Add for ComplexNums<i32> {
    type Output = ComplexNums<i32>;

    //a+b
    fn add(self, rhs: Self) -> Self::Output {
        ComplexNums {
            re: self.re + rhs.re,
            im: self.im + rhs.im
        }
    }
}

pub fn run() {
    let mut a = ComplexNums::new(1, 2);
    let mut b = ComplexNums::new(3, 4);
    println!("{:?}", a+b);
}
