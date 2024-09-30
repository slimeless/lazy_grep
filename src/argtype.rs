use std::path::Path;

pub enum ArgType {
    File,
    Directory,
    Stdin,
}

pub trait GetArgType {
    fn get_argtype(&self) -> ArgType;
}

impl GetArgType for Path {
    fn get_argtype(&self) -> ArgType {
        if self.exists() {
            if self.is_file() {
                ArgType::File
            } else {
                ArgType::Directory
            }
        } else {
            ArgType::Stdin
        }
    }
}
