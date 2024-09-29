use std::{
    error::Error,
    fs::{self, File},
    io::{BufRead, BufReader},
    path::Path,
};

use owo_colors::OwoColorize;
use regex::Regex;

use crate::argtype::{ArgType, GetArgType};

pub trait GetContent {
    fn get_content(&self, re: &Regex, prefix: String) -> Result<Vec<String>, Box<dyn Error>>;
}

impl GetContent for String {
    fn get_content(&self, re: &Regex, prefix: String) -> Result<Vec<String>, Box<dyn Error>> {
        let mut result = Vec::<String>::new();
        for line in self.lines() {
            if re.is_match(line) {
                result.push(format!("{}{}", prefix.bold().magenta(), line));
            }
        }
        Ok(result)
    }
}

impl GetContent for File {
    fn get_content(&self, re: &Regex, prefix: String) -> Result<Vec<String>, Box<dyn Error>> {
        let reader = BufReader::new(self);
        let mut result = Vec::<String>::new();
        for line in reader.lines().map_while(Result::ok) {
            if re.is_match(&line) {
                result.push(format!("{}{}", prefix.bold().magenta(), line));
            }
        }
        Ok(result)
    }
}

impl GetContent for Path {
    fn get_content(&self, re: &Regex, prefix: String) -> Result<Vec<String>, Box<dyn Error>> {
        let mut result = Vec::<String>::new();
        for entry in (fs::read_dir(self)?).flatten() {
            match entry.path().get_argtype() {
                ArgType::File => {
                    let file = File::open(entry.path())?;
                    let pref = format!("{}: ", entry.path().display().to_string().bold().magenta());
                    result.append(&mut file.get_content(&re.clone(), pref)?);
                }
                ArgType::Directory => {
                    result.append(&mut entry.path().get_content(&re.clone(), prefix.clone())?);
                }
                ArgType::Stdin => {}
            };
        }

        Ok(result)
    }
}
