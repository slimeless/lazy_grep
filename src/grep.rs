use std::{
    error::Error,
    fs::{self, File},
    io::{BufRead, BufReader},
    path::Path,
};

use regex::Regex;

use crate::argtype::{ArgType, GetArgType};

pub trait GetContent {
    fn get_content(&self, re: Regex) -> Result<Vec<String>, Box<dyn Error>>;
}

impl GetContent for String {
    fn get_content(&self, re: Regex) -> Result<Vec<String>, Box<dyn Error>> {
        let mut result = Vec::<String>::new();
        for line in self.lines() {
            if re.is_match(line) {
                result.push(line.to_string());
            }
        }
        Ok(result)
    }
}

impl GetContent for File {
    fn get_content(&self, re: Regex) -> Result<Vec<String>, Box<dyn Error>> {
        let reader = BufReader::new(self);
        let mut result = Vec::<String>::new();
        for line in reader.lines().map_while(Result::ok) {
            if re.is_match(&line) {
                result.push(line);
            }
        }
        Ok(result)
    }
}

impl GetContent for Path {
    fn get_content(&self, re: Regex) -> Result<Vec<String>, Box<dyn Error>> {
        let mut result = Vec::<String>::new();
        for entry in (fs::read_dir(self)?).flatten() {
            match entry.path().get_argtype() {
                ArgType::File => {
                    let file = File::open(entry.path())?;
                    result.append(&mut file.get_content(re.clone())?);
                }
                ArgType::Directory => {
                    result.append(&mut entry.path().get_content(re.clone())?);
                }
                ArgType::Stdin => {}
            };
        }

        Ok(result)
    }
}
