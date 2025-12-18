// Add modules
mod Lexer {
    pub mod Token;
    pub mod Tokeniser;
}

// Import specific functions from libraries/modules
use std::fs; // Read files
use std::io::BufRead;

fn main() {
    println!("Hello, world!");
    let mut input = String::new();
	let tokens: Vec<i8> = Vec::new();

    // Lexer
    let mut _filePath : String = String::from("DIY.txt");
    // Read file
    let _input : String = fs::read_to_string("DIY.txt").expect(&concat_strings("Unable to read ", _filePath));
    // Tokenise file input
    let _tokens : Vec<Lexer::Token::Token> = Lexer::Tokeniser::tokenise(_input);

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

pub fn concat_strings(_string1 : &str, _string2 : String) -> String { return format!("{_string1}{_string2}"); }
