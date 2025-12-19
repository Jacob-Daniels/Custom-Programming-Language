// Add modules
mod Core {
    pub mod Strings;
}
mod Lexer {
    pub mod Token;
    pub mod Tokeniser;
}

// Import specific functions from libraries/modules
use std::fs; // Read files
use std::io::{self, BufReader, BufRead};

fn main() {
    println!("Hello, world!");
    let mut input = String::new();
	let tokens: Vec<i8> = Vec::new();

    // Lexer
    let _filePath : String = String::from("src/DIY.txt");
    // Read file

    let _file = std::fs::File::open(&_filePath);
    
    // FIX THIS LINE: _FILE IS NOT RECOGNISED BY BUFREADER...
    let _fileReader = std::io::BufReader::new(_file);


    //let _input : String = fs::read_to_string(&_filePath).expect(&Core::Strings::concat_strings("Unable to read ", &_filePath));
    // Tokenise file input
    //let _tokens : Vec<Lexer::Token::Token> = Lexer::Tokeniser::tokenise(_input);

    let mut inputReader: std::io::StdinLock<'static> = std::io::stdin().lock();
    input.clear();
    match inputReader.read_line(&mut input) {
        Ok(n) => {
            println!("Working: {n} bytes read from {input}");
        }
        Err(error) => {
            println!("Not working: {error}");
        }
    }
}

