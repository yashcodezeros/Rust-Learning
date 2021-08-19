pub mod greetings{
   pub mod english;
   pub mod french;
}

#[test] //test the fn with "cargo build && cargo test" in crate/dependency project
// #[should_panic]  
// #[ignore]
pub fn english_greeting_correct(){
    assert_eq!("hello",greetings::english::hello());
}