// Arrays - Fixed list where elements are the same data types
use std::mem;
pub fn run(){

    let mut numbers: [i32;6] = [1, 2, 3, 4, 5, 6];

    //re-assign values
    numbers[4] = 40;

    println!("{:?}",numbers);

    println!("Single value: {}",numbers[0]);

    //get array length
    println!("Array Length: {}",numbers.len());

    //arrays are stack allocated
    println!("Array occupies {} bytes",mem::size_of_val(&numbers));

    //get slice 
    let slice: &[i32] = &numbers[3..5];
    println!("Slice: {:?}",slice);

    let x  = [1u16;10]; //want "1" 10 times in array we cam give size of variable in here also.
    println!("x = {:?}",x);

    for i in 0..x.len(){  //x.len() = 10
        println!("x{} = {}",i,x[i]);
    }

    //arrays are stack allocated
    println!("Array x occupies {} bytes",mem::size_of_val(&x));

    //matrix (array of array)
    let mtx: [[f32;3];3] = [
        [1.0,0.0,0.0],
        [0.0,2.0,0.0],
        [0.0,2.0,3.0]
    ];
    println!("{:?}",mtx);

    //find diagonal of matrix
    for i in 0..mtx.len(){
        for j in 0..mtx[i].len(){
            if i == j {
                println!("Matrix[{}][{}] = {}",i,j,mtx[i][j]);
            }
        }
    }
}