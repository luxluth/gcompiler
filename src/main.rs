mod args;
use args::{QArgs, Entity};
use clap::Parser;

mod parser;

use std::{path::PathBuf, process::exit};

fn get_file_content(path: &PathBuf) -> String {
    let content = std::fs::read_to_string(path);
    match content {
        Ok(c) => {
            return c;
        },
        Err(e) => {
            println!("[ERROR]: {e} '{p}'", e=e, p=path.to_str().unwrap_or("unknown path"));
            exit(1)
        },
    }
}


fn main() {
    let args = QArgs::parse();
    let path = match args.entity {
        Entity::Compile(_c) => {
            PathBuf::from(_c.path)
        },
    };

    let content = get_file_content(&path);
    if content.len() == 0 {
        return;
    }

    let mut lexer = parser::Lexer::new(content);
    let tokens = lexer.tokenize();
    match tokens {
        Ok(t) => {
            println!("{:?}", t);
        },
        Err(e) => {
            println!("[ERROR]: {}", e);
            exit(1);
        },
    }
}
