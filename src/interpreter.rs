/*
#define x
    min 0
    max 100
    name "x"
#end

#define y
    min 0
    max 100
    name "y"
#end

#root
    box (0, 0, 100, 100)
    color 0x000000
    background 0xffffff
    axes (x, y)
#end

#grid
    color 0x000000
    alpha 0.2
    thickness 1
#end

@line
    from (0, 0)
    to (100, 100)
    name "line"
    color 0x000000
#end

@graph
    name "x^2"
    color 0xff0000
    thickness 2
    function x^2
#end

@point
    x 50
    y 50
    name "A"
    color 0x0000ff
#end
*/

/*
Gramatical rules
- A file is composed of multiple declarations
- A declaration is composed of a keyword and a block
- A block is composed ends with a end keyword
- A line is composed of a keyword and a value
- Any statement ends with a semicolon
*/

use crate::parser;
use crate::parser::{Token, TokenType};
use crate::parser::{
    TOP_LEVEL_DECLARATIONS,
    INNER_FUNCTIONS,
};

use std::process::exit;

#[derive(Debug, Clone)]
pub struct Declaration {
    pub varname: String,
    pub name: String,
    pub min: f64,
    pub max: f64,
}

#[derive(Debug, Clone)]
pub struct Root {
    pub _box: (f64, f64, f64, f64),
    pub color: String,
    pub background: String,
    pub axes: (Declaration, Declaration),
}

#[derive(Debug, Clone)]
pub struct Grid {
    pub color: String,
    pub alpha: f64,
    pub thickness: f64,
}

#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub args: Vec<Arg>,
}

#[derive(Debug, Clone)]
pub enum Arg {
    Min(f64),
    Max(f64),
    Name(String),
    Color(String),
    Background(String),
    Alpha(f64),
    Thickness(f64),
    Function(Function),
    From(f64),
    To(f64),
    Axes((String, String)),
    Box((f64, f64, f64, f64)),
}

pub struct Interpreter {
    pub tokens: Vec<Token>,
    pub position: usize,
    pub definitions: Vec<Declaration>,
}

impl Interpreter {
    pub fn new(input: String) -> Self {
        let mut lexer = parser::Lexer::new(input.clone());
        let tokens = lexer.tokenize();
        match tokens {
            Ok(t) => {
                Interpreter {
                    tokens: t,
                    position: 0,
                    definitions: Vec::new(),
                }
            },
            Err(e) => {
                println!("[ERROR]: {}", e);
                exit(1);
            },
        }
    }

    fn consume(&mut self, len: usize) {
        self.position += len;
    }

    fn next(&mut self) -> Option<Token> {
        if self.position >= self.tokens.len() {
            return None;
        }
        let token = self.tokens[self.position].clone();
        Some(token)
    }

    fn get_token(&mut self, position: usize) -> Option<Token> {
        if position >= self.tokens.len() {
            return None;
        }
        let token = self.tokens[position].clone();
        Some(token)
    }

    //// Functions for checking if each declaration is valid
    //// has an end keyword
    fn check_declarations(&mut self) {
        let mut current_pos = 0;
        let mut current_token = self.get_token(current_pos);
        while current_token.is_some() {
            let token = current_token.unwrap();
            if token.token_type == TokenType::KEYWORD {
                if TOP_LEVEL_DECLARATIONS.contains(&token.value.as_str()) || INNER_FUNCTIONS.contains(&token.value.as_str()) {
                    let mut end_found = false;
                    let mut end_pos = current_pos + 1;
                    let mut end_token = self.get_token(end_pos);
                    while end_token.is_some() {
                        let token = end_token.unwrap();
                        if token.token_type == TokenType::KEYWORD && token.value == "end" {
                            end_found = true;
                            break;
                        }
                        end_pos += 1;
                        end_token = self.get_token(end_pos);
                    }
                    if !end_found {
                        println!("[ERROR]: Missing 'end' keyword for declaration '{}' at line {}", token.value, token.line);
                        exit(1);
                    }
                }
            }
            current_pos += 1;
            current_token = self.get_token(current_pos);
        }
        
    }

    //// Functions for checking if the root is present and there is only one
    fn check_root(&mut self) {
        let mut root_found = false;
        let mut current_pos = 0;
        let mut current_token = self.get_token(current_pos);
        while current_token.is_some() {
            let token = current_token.unwrap();
            if token.token_type == TokenType::DECLARATION && token.value == "root" {
                if root_found {
                    println!("[ERROR]: Multiple 'root' declarations");
                    exit(1);
                }
                root_found = true;
            }
            current_pos += 1;
            current_token = self.get_token(current_pos);
        }
        if !root_found {
            println!("[ERROR]: Missing 'root' declaration");
            exit(1);
        }
    }

    fn seek_to(&mut self, value: String, token_type: TokenType) -> Option<usize> {
        let mut current_pos = self.position;
        let mut current_token = self.get_token(current_pos);
        while current_token.is_some() {
            let token = current_token.unwrap();
            if token.token_type == token_type && token.value == value {
                return Some(current_pos);
            }
            current_pos += 1;
            current_token = self.get_token(current_pos);
        }

        None
    }

    /// This function assumes that the root is present and there is only one
    fn get_root(&mut self) {
        let mut _box = (0.0, 0.0, 0.0, 0.0);
        let mut has_box = false;
        let mut color = String::new();
        let mut has_color = false;
        let mut background = String::new();
        let mut has_background = false;
        let mut x_axe = Some(self.definitions[0].clone());
        let mut y_axe = Some(self.definitions[1].clone());
        let mut has_axes = false;
        let mut current_pos = self.seek_to("root".to_string(), TokenType::DECLARATION).unwrap() + 1;
        let mut current_token = self.get_token(current_pos);
        while current_token.is_some() {

            let token = current_token.unwrap();
            if token.token_type == TokenType::KEYWORD {
                // TODO
                if token.value == "box" {}
            }

            current_pos += 1;
            current_token = self.get_token(current_pos);
        }
    }
    
    /// Functions for processing the definitions
    /// Finds all the definitions and stores them in the definitions vector
    /// Each definition is composed of a name, a min and a max
    fn process_definitions(&mut self) {
        let mut current_pos = 0;
        let mut current_token = self.get_token(current_pos);
        while current_token.is_some() {
            let token = current_token.unwrap();
            if token.token_type == TokenType::DEFINE {
                let mut varname = String::new();
                let mut name = String::new();
                let mut has_name = false;
                let mut min = 0.0;
                let mut has_min = false;
                let mut max = 0.0;
                let mut has_max = false;

                let mut current_pos = current_pos + 1;
                let mut current_token = self.get_token(current_pos);
                while current_token.is_some() {
                    let token = current_token.unwrap();
                    if token.token_type == TokenType::KEYWORD {
                        if token.value == "max" {
                            has_max = true;
                            // step to the next token
                            current_pos += 1;
                            current_token = self.get_token(current_pos);
                            if current_token.is_none() {
                                println!("[ERROR]: Missing value for 'max' keyword");
                                exit(1);
                            }
                            let token = current_token.unwrap();
                            if token.token_type == TokenType::INTERGER || token.token_type == TokenType::FLOAT {
                                max = token.value.parse::<f64>().unwrap();
                            } else {
                                println!("[ERROR]: Invalid value for 'max' keyword, expected a number got '{}'", token.value);
                                exit(1);
                            }
                        }

                        if token.value == "min" {
                            has_min = true;
                            // step to the next token
                            current_pos += 1;
                            current_token = self.get_token(current_pos);
                            if current_token.is_none() {
                                println!("[ERROR]: Missing value for 'min' keyword");
                                exit(1);
                            }
                            let token = current_token.unwrap();
                            if token.token_type == TokenType::INTERGER || token.token_type == TokenType::FLOAT {
                                min = token.value.parse::<f64>().unwrap();
                            } else {
                                println!("[ERROR]: Invalid value for 'min' keyword, expected a number got '{}'", token.value);
                                exit(1);
                            }
                        }

                        if token.value == "name" {
                            has_name = true;
                            // step to the next token
                            current_pos += 1;
                            current_token = self.get_token(current_pos);
                            if current_token.is_none() {
                                println!("[ERROR]: Missing value for 'name' keyword");
                                exit(1);
                            }
                            let token = current_token.unwrap();
                            if token.token_type != TokenType::STRING {
                                println!("[ERROR]: Invalid value for 'name' keyword");
                                exit(1);
                            }
                            name = token.value.clone();
                        }
                    }

                    if token.token_type == TokenType::VARNAME {
                        varname = token.value.clone();
                    }

                    if token.token_type == TokenType::DECLARATION && token.value == "end" {
                        if !has_min {
                            println!("[ERROR]: Missing 'min' keyword for definition '{}'", varname);
                            exit(1);
                        }
                        if !has_max {
                            println!("[ERROR]: Missing 'max' keyword for definition '{}'", varname);
                            exit(1);
                        }
                        if !has_name {
                            println!("[ERROR]: Missing 'name' keyword for definition '{}'", varname);
                            exit(1);
                        }
                        let definition = Declaration {
                            varname,
                            name,
                            min,
                            max,
                        };
                        self.definitions.push(definition);
                        break;
                    }

                    current_pos += 1;
                    current_token = self.get_token(current_pos);
                }
            }

            current_pos += 1;
            current_token = self.get_token(current_pos);
        }

    }

    /**
     * Preprocess the code
     * This step is for:
     * - Track syntax errors
     * - Check if the code is valid
     */
    fn preprocess(&mut self) {
        self.check_declarations();
        self.check_root();
        self.process_definitions();
        println!("{:?}", self.definitions);
        self.get_root();
    }
    pub fn compile(&mut self) {
        self.preprocess();
    }



    //// Functions for the generation of the SVG string
    fn gen_svg(&mut self) {}
    fn compute_line(&mut self, pos_from: (f64, f64), pos_to: (f64, f64)) {}
    fn compute_graph(&mut self, f: String) {}

}
