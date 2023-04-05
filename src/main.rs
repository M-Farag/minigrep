use std::env;
use std::process;
use minigrep as mg;

fn main(){
    let arguments:Vec<String> = env::args().collect();
    let config = mg::Config::new(&arguments).unwrap_or_else(
        |err| {
            eprintln!("Error is: {}",err);
            process::exit(1);
        }
    );

    // we used if let because we don't care about the Ok() case as it returns a unit type
    if let Err(e) = mg::run(config) {
        eprintln!("Error is: {}",e);
        process::exit(1);
    }

}

