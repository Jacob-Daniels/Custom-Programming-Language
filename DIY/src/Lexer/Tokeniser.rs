use crate::Core::Strings;
use crate::Lexer::Token::Token;

use std::fs::read_to_string;

pub fn tokenise(_input : String) -> Vec<Token> {
	// Initialise vector of tokens
	let mut _tokens : Vec<Token> = Vec::new();

	println!("Input: {}", _input);




	return _tokens;



	//// Loop _input: line by line
	////for line in read_to_string(_input).unwrap().lines()
	//for line in read_to_string(_input).unwrap().lines()
	//{
	//	println!("{}", line);
	//	//// Loop individual line
	//	//for (i, c) in line.chars().enumerate()
	//	//{
	//	//	//let _string1 = "Char: ";
	//	//	//let print =  &Strings::concat_strings(&Strings::concat_strings("Index: ", &i.to_string()), &Strings::concat_strings("Char: ", &c.to_string()));
	//	//	//println!("{}", print);
	//	//}
	//}

	//println!("Tokenised!");
	//return _tokens;
}
