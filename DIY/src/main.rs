// Add modules
mod Lexer {
    pub mod Tokeniser;
}

// Import specific functions from libraries/modules
use std::io::BufRead;

fn main() {
    println!("Hello, world!");
    let mut input = String::new();
	let tokens: Vec<i8> = Vec::new();

    // Lexer
    Lexer::Tokeniser::Tokenise();

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
