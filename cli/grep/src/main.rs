use std::env;           //Environment library  collect the variables through commands
use std::process;       // for  process::exit(1) that terminates the program with a status quo

use grep::Config;

fn main() {
    //It takes command line as input and store them in vector args
    let args: Vec<String> = env::args().collect();

    // ---unwrap_or_else :- ------[USED IN CASES OF RESULT ENUMS RETURNS]------
    // 1. In the Ok() case it will return the value stored in Ok.
    // 2. In the Err case ,it will execute the closure passing the error.
    // 3. Inside the closure it will printout the error and terminate the program with the status code.

    let config = Config::new(&args).unwrap_or_else( |err: &str| {
        eprintln!("Problem parsing arguments : {}",err);
        process::exit(1);
    });

    println!("Searching for {:?} ", config.query);
    println!("In file {:?} ",config.filename);

    if let Err(e) = grep::run(config) {
        eprintln!("Application error: {}",e);
        process::exit(1);
    }
 }



