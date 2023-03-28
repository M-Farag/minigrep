#[derive(Debug)]
pub struct Config{
    file_path:String,
    query:String
}

impl Config {
    pub fn new(file_path:String, query:String) -> Self
    {
        Self { file_path, query }
    }

    pub fn file_path(&self) -> &String
    {
        &self.file_path
    }
}

pub fn parse_config(arguments:&Vec<String>) ->Config { 
    let config = Config::new(
     arguments[1].clone(),
     arguments[2].clone()
    );

    config
}


#[cfg(test)]
mod main_lib_tests {
    use super::*;

    #[test]
    fn test_parse_config_works() {
        let args:Vec<String> = vec![String::from("one"),String::from("file_path"),String::from("query")];

        let config = parse_config(&args);
        assert_eq!(config.file_path,"file_path");
    }
    
    #[test]
    #[should_panic(expected="index out of bounds")]
    fn test_parse_config_panic_if_arguments_are_empty() {
        let args:Vec<String> = vec![];
        parse_config(&args);
    }

}