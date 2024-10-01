use std::{
    fmt::Display,
    io::{stdin, Read},
    str::FromStr,
};

#[derive(Debug, Clone)]
pub struct MaybeStdin(String);

impl FromStr for MaybeStdin {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "-" {
            let mut buff = String::new();
            let _ = stdin()
                .read_to_string(&mut buff)
                .map_err(|_| "failed read to string");
            if buff.trim().is_empty() {
                return Err("stdin is clear");
            }
            Ok(MaybeStdin(buff))
        } else {
            Ok(MaybeStdin(s.to_string()))
        }
    }
}

impl Display for MaybeStdin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
