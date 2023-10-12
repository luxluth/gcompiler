mod args;
use args::{QArgs, Entity};
use clap::Parser;

mod parser;
mod interpreter;
use interpreter::Interpreter;

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

    let mut interpreter = Interpreter::new(content);
    interpreter.compile();
}
