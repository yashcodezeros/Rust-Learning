//! This module contains English Phrases
//! rustdoc filename.rs generates doc for the comments we provided and u can see by running the index.html
//! # Examples
//!```
//! let username = "Adam";
//! println!("{} {}!",
//!    phrases::greetings::english::hello(),
//!    username);
//!```

/// Applies to code that follows it.
/// In this case, it's our `hello()` function.
pub fn hello() -> String { 
    "hello".to_string() /*here*/ 
}

pub fn good_bye() -> String {
    "goodbye".to_string() 
}