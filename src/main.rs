use std::env; //std::env::args will panic if any argument contains invalid Unicode
use std::fs;
fn main() {
    let args: Vec<String> = env::args().collect(); //collect method is used to turn iterator into
    //a vector; here a vector of string
    //dbg!(args);
    let query = &args[1];
    let file_path = &args[2];

    println!("Searching for {query}");
    println!("In file {file_path}");

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    println!("With text:\n{contents}");
}
