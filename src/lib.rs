use std::{error::Error, fs};

#[derive(Debug)]
pub struct Config{
    file_path:String,
    query:String
}

impl Config {
    pub fn new(arguments:&Vec<String>) -> Result<Config,&str>
    {
        if arguments.len() < 3 
        {
            return Err("Expecting arguments to be at least 3");
        }
        Ok(Config { file_path:arguments[1].clone(), query:arguments[2].clone() })
    }

    pub fn query(&self) -> &String
    {
        &self.query
    }

    pub fn file_path(&self) -> &String
    {
        &self.file_path
    }
}

pub fn run(config:Config) -> Result<(),Box<dyn Error>>
{
    let contents = fs::read_to_string(config.file_path())?;
    for line in search(&config.query, contents.as_str())
    {
        println!("{line}");
    }
    Ok(())
}

fn search<'a>(query:&str, contents:&'a str) -> Vec<&'a str>
{
    let mut results: Vec<&str> = Vec::new();
    for line in contents.lines()
    {
        if line.contains(query){
            results.push(line)
        }
    }
    results
}


#[cfg(test)]
mod main_lib_tests {
    use super::*;

    #[test]
    fn test_struct_config_constructor_works()
    {
        let args:Vec<String> = vec![String::from("0"),String::from("1"),String::from("2")];
        let config = Config::new(&args).unwrap();

        assert_eq!(&String::from("1"),config.file_path());
        assert_eq!(&String::from("2"),config.query());
    }

    #[test]
    #[should_panic(expected="Expecting arguments to be at least 3")]
    fn test_config_constructor_panics_with_fewer_arguments() 
    {
        let args:Vec<String> = vec![];
        let config = Config::new(&args).unwrap();
    }

    #[test]
    fn test_config_first_argument_is_file_path()
    {
        let args:Vec<String> = vec![String::from("0"),String::from("1"),String::from("2")];
        let config = Config::new(&args).unwrap();

        assert_eq!(&String::from("1"),config.file_path());
    }

    #[test]
    fn test_config_second_argument_is_query()
    {
        let args:Vec<String> = vec![String::from("0"),String::from("1"),String::from("2")];
        let config = Config::new(&args).unwrap();

        assert_eq!(&String::from("2"),config.query());
    }

    #[test]
    fn test_run_returns_ok_with_valid_file_path()
    {
        let args:Vec<String> = vec![String::from("0"),String::from("src/main.rs"),String::from("2")];
        let config = Config::new(&args).unwrap();

        assert!(run(config).is_ok());
    }

    #[test]
    fn test_run_returns_err_with_invalid_file_path()
    {
        let args:Vec<String> = vec![String::from("0"),String::from("src/main.rs1"),String::from("2")];
        let config = Config::new(&args).unwrap();

        assert!(run(config).is_err());
    }

    #[test]
    fn test_search_returns_valid_result()
    {
        let query = "duct";
        let contents = "\
        Hello world
        I am a rustacean
        I love rust
        I am productive in rust
        I have a duct tape
        ";
        assert_eq!(vec!["        I am productive in rust", "        I have a duct tape"],search(query,contents));
    }

}