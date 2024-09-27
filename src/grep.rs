use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub trait GetContent {
    fn get_content(&self) -> Box<dyn Iterator<Item = String> + '_>;
}

impl GetContent for String {
    fn get_content(&self) -> Box<dyn Iterator<Item = String> + '_> {
        let content = &self.lines();
        Box::new(content.clone().map(|l| l.to_string()))
    }
}

impl GetContent for File {
    fn get_content(&self) -> Box<dyn Iterator<Item = String> + '_> {
        let reader = BufReader::new(self);
        Box::new(reader.lines().map_while(Result::ok))
    }
}
