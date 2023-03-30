use std::env;
use std::error::Error;
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

    // we used if let because we don't care about the Ok() case as it returns a unit type
    if let Err(e) = run(config) {
        println!("Error is: {}",e);
        process::exit(1);
    }

}

fn run(config:mg::Config) -> Result<(),Box<dyn Error>>
{
    let contents = fs::read_to_string(config.file_path())?;
    println!("Contents are: {:#?}",contents);
    Ok(())
}