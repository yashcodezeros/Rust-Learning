use std::thread;
use std::time;
use std::collections::HashSet; 

pub fn run(){

    let mut greeks = HashSet::new();
    greeks.insert("Gamma");
    greeks.insert("Alpha");
    println!("{:?}",greeks);
   let added_delta = greeks.insert("Delta");
    if added_delta{
        println!("Delta added!!")
    }
    if !greeks.contains("beta"){
        println!("don't have beta!!")
    }

    let removed = greeks.remove("Delta");
    if removed{
        println!("Delta removed!!")
    }
    println!("{:?}",greeks);

    //Hashsets
    let _1_5: HashSet<_> = (1..=5).collect();
    let _6_10: HashSet<_> = (6..=10).collect();
    let _1_10: HashSet<_> = (1..=10).collect();
    let _2_8: HashSet<_> = (2..=8).collect();

    //check the set is subset of any set
    println!("is {:?} set is subset of {:?} set? ans = {:?}",_2_8,_1_10,_2_8.is_subset(&_1_10));

    //disjoint = no common element
    println!("is {:?} set is disjoint from {:?} set? ans = {:?}",_1_5,_6_10,_1_5.is_disjoint(&_6_10));

    //union = get all the value from sets
    //intersection = get common from both sets
    println!("get all values from {:?} set and from {:?} set. ans = {:?}",_2_8,_6_10,_2_8.union(&_6_10));
    println!("get common values from {:?} set and from {:?} set. ans = {:?}",_1_10,_2_8,_1_10.intersection(&_2_8));

}