use std::env;
use std::fs;
use minigrep as mg;
fn main(){
    let arguments:Vec<String> = env::args().collect();
    let config = mg::Config::new(&arguments).unwrap();


    let contents = fs::read_to_string(config.file_path());
    println!("Contents are: {:#?}",contents);

}