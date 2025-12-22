// Add modules
mod Core {
    pub mod FileReader;
    pub mod Strings;
}
mod Lexer {
    pub mod Token;
    pub mod Tokeniser;
}

// Import specific functions from libraries/modules
use std::fs; // Read files
use std::io::{self, BufReader, BufRead};
use std::ptr::null;
use crate::Core::FileReader::FileReader;

fn main() -> io::Result<()> {
    println!("Hello, world!");
    let mut input = String::new();
	let tokens: Vec<i8> = Vec::new();

    // Lexer
    let _filePath : String = String::from("src/DIY.txt");
    // Read file

    // Custom file reader


    let mut fileReader = FileReader::open_file(_filePath)?;

    // Loop lines
    let mut buffer = String::new();
    while let Some(line) = fileReader.read_line(&mut buffer) {
        println!("{}", line?.trim());
    }

    Ok(())
}

