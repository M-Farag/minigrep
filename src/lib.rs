#[derive(Debug)]
pub struct Config{
    file_path:String,
    query:String
}

impl Config {
    pub fn new(arguments:&Vec<String>) -> Self
    {
        Self { file_path:arguments[1].clone(), query:arguments[2].clone() }
    }

    pub fn file_path(&self) -> &String
    {
        &self.file_path
    }
}


#[cfg(test)]
mod main_lib_tests {
    use super::*;

}