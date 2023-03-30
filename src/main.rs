use std::env;
use std::fs;
use std::process;
use minigrep as mg;
fn main(){
    let arguments:Vec<String> = env::args().collect();
    let config = mg::Config::new(&arguments).unwrap_or_else(
        |err| {
            println!("Error is: {}",err);
            process::exit(1);
        }
    );


    let contents = fs::read_to_string(config.file_path());
    println!("Contents are: {:#?}",contents);

}