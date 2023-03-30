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
    println!("Contents are: {:#?}",contents);
    Ok(())
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

}