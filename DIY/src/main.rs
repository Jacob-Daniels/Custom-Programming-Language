use std::io::BufRead;

fn main() {
    println!("Hello, world!");
    let mut input = String::new();

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
