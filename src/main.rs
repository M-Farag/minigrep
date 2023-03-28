use std::env;
use std::fs;
use minigrep as mg;
fn main(){
    let arguments:Vec<String> = env::args().collect();

    let config = mg::parse_config(&arguments);


    let contents = fs::read_to_string(config.file_path());
    println!("Contents are: {:#?}",contents);

}