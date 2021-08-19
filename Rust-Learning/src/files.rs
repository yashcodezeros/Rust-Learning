use std::fs;

pub fn run() -> std::io::Result<()>{ fs::write("calvin.txt","hello")?; Ok(()) }
