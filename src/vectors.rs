// Vectors - Resizable arrays
use std::mem;
pub fn run(){

    let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5, 6];

    //re-assign values
    numbers[4] = 40;

    //add on to vector
    numbers.push(5);
    numbers.push(6);

    //pop off last value
    numbers.pop();

    println!("{:?}",numbers);

    println!("Single value: {}",numbers[0]);

    //get Vectors length
    println!("Vector Length: {}",numbers.len());

    //Vectors are stack allocated
    println!("Vector occupies {} bytes",mem::size_of_val(&numbers));

    //get slice 
    let slice: &[i32] = &numbers[3..5];
    println!("Slice: {:?}",slice);

    //loop through vector values
    for x in numbers.iter(){
        println!("Number: {}",x)
    }

    //loop and mutate values
    for x in numbers.iter_mut(){
        *x *= 2;
    }

    println!("Vector Numbers Multipled by 2: {:?}",numbers);

    //reverse 
    let mut nums: Vec<i32> = vec![1, 2, 3, 4, 5, 6];
    for x in nums.iter_mut().rev(){
        println!("in reverse: {:?}",x);
    }

    //into_iter and extend

    let mut vec = vec![3,2,1];
    let mut vec2 = vec![3,2,1];

    // let it  = vec.into_iter(); //into_iter uses to fetch all the values and store in new var then the older var does not have the values.
                                //you cannot use nor "vec" variable.
    vec2.extend(vec);// with extend vec2 var aquired and stored vec's all values. 
    println!("Vec: {:?}",vec2);

    let mut a = Vec::new();
    a.push(1);
    a.push(2);
    a.push(3);
    a.push(4);
    println!("New Vec: {:?}",a);
    a.push(44);
    println!("New Updated Vec: {:?}",a);
    let idx:usize = 0;
    a[idx] = 321;
    println!("IDX: {}",a[idx]);

    //option
    match a.get(3)
    {
        Some(x) => println!("a[3] = {}",x),
        None => println!("error, no such element!")
    }

    //it will pop the value with iteration and empty the vector array and stop after the vector is empty
    while let Some(x) = a.pop(){
        println!("{}",x);
    }
    println!("After Pop the vector: {:?}",a);
}