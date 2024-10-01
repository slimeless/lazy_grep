use std::{
    error::Error,
    fs::{self, File},
    io::{BufRead, BufReader},
    path::Path,
};

use owo_colors::OwoColorize;
use regex::Regex;

use crate::{
    argtype::{ArgType, GetArgType},
    view::Highlight,
};

pub trait GetContent {
    fn get_content(&self, re: &Regex) -> Result<Vec<String>, Box<dyn Error>>;
}
pub trait GetContentWithPrefix: GetContent {
    fn get_prefix_content(
        &self,
        re: &Regex,
        prefix: String,
    ) -> Result<Vec<String>, Box<dyn Error>> {
        let content = self.get_content(re)?;
        let content_with_prefix = content
            .iter()
            .map(|item| format!("{prefix}: {item}"))
            .collect();
        Ok(content_with_prefix)
    }
}
impl GetContentWithPrefix for File {}

impl GetContent for String {
    fn get_content(&self, re: &Regex) -> Result<Vec<String>, Box<dyn Error>> {
        let mut result = Vec::<String>::new();
        for line in self.lines() {
            if re.is_match(line.trim()) {
                result.push(line.to_string().highlight(re));
            }
        }
        Ok(result)
    }
}

impl GetContent for File {
    fn get_content(&self, re: &Regex) -> Result<Vec<String>, Box<dyn Error>> {
        let reader = BufReader::new(self);
        let mut result = Vec::<String>::new();
        for line in reader.lines().map_while(Result::ok) {
            if re.is_match(&line) {
                result.push(line.to_string().highlight(re));
            }
        }
        Ok(result)
    }
}

impl GetContent for Path {
    fn get_content(&self, re: &Regex) -> Result<Vec<String>, Box<dyn Error>> {
        let mut result = Vec::<String>::new();
        for entry in (fs::read_dir(self)?).flatten() {
            match entry.path().get_argtype() {
                ArgType::File => {
                    let file = File::open(entry.path())?;
                    let pref = format!("{}", entry.path().display().to_string().bold().magenta());
                    result.append(&mut file.get_prefix_content(&re.clone(), pref)?);
                }
                ArgType::Directory => {
                    result.append(&mut entry.path().get_content(&re.clone())?);
                }
                ArgType::Stdin => {}
            };
        }

        Ok(result)
    }
}
