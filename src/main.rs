use std::env; //std::env::args will panic if any argument contains invalid Unicode
use std::process;

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect(); //collect method is used to turn iterator into
    //a vector; here a vector of string
    //dbg!(args);

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    //error handling

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    //iuse if let for error handling
    if let Err(e) = minigrep::run(config) {
        //run config function
        println!("Application error: {e}");
        process::exit(1);
    }
}
