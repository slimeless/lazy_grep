use std::{
    fmt::Display,
    io::{stdin, Read},
    path::{Path, PathBuf},
    str::FromStr,
};

#[derive(Debug, Clone)]
pub enum MaybeStdin {
    String(String),
    Path(PathBuf),
}

impl FromStr for MaybeStdin {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "-" {
            let mut buff = String::new();
            let _ = stdin()
                .read_to_string(&mut buff)
                .map_err(|_| "failed read to string");
            Ok(MaybeStdin::String(buff))
        } else {
            let path = Path::new(s);
            if path.exists() {
                Ok(MaybeStdin::Path(path.to_path_buf()))
            } else {
                Err("Provied path is invalid")
            }
        }
    }
}

impl Display for MaybeStdin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MaybeStdin::String(str) => write!(f, "{str}"),
            MaybeStdin::Path(path) => write!(f, "{}", path.display()),
        }
    }
}
