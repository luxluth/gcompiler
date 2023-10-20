use crate::parser;
use crate::parser::{Token, TokenType};
use crate::parser::{
    TOP_LEVEL_DECLARATIONS,
    INNER_FUNCTIONS,
};

use exmex::prelude::*;

use std::process::exit;

#[derive(Debug, Clone)]
pub struct Declaration {
    pub varname: String,
    pub name: Option<String>,
    pub min: Option<f64>,
    pub max: f64,
}

#[derive(Debug, Clone)]
pub struct Root {
    pub _box: (f64, f64, f64, f64),
    pub color: String,
    pub background: String,
    pub axis: (Declaration, Declaration),
}

#[derive(Debug, Clone)]
pub struct Grid {
    pub color: String,
    pub alpha: Option<f64>,
    pub thickness: Option<f64>,
    pub step: Option<f64>,
}

#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub args: Vec<Arg>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Arg {
    Name(String),
    Func(String),
    Color(String),
    Thickness(f64),
    From((f64, f64)),
    To((f64, f64)),
    At((f64, f64)),
}

pub struct Interpreter {
    pub lexer: parser::Lexer,
    pub tokens: Vec<Token>,
    pub position: usize,
    pub definitions: Vec<Declaration>,
    pub root: Option<Root>,
    pub grid: Option<Grid>,
    pub functions: Vec<Function>,
}

impl Interpreter {
    pub fn new(input: String) -> Self {
        let mut lexer = parser::Lexer::new(input);
        let tokens = lexer.tokenize();
        match tokens {
            Ok(t) => {
                Interpreter {
                    lexer,
                    tokens: t,
                    position: 0,
                    definitions: Vec::new(),
                    functions: Vec::new(),
                    root: None,
                    grid: None,
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

    fn get_var(&mut self, name: String) -> Option<Declaration> {
        for declaration in self.definitions.iter() {
            if declaration.varname == name {
                return Some(declaration.clone());
            }
        }
        None
    }
    
    /// Functions for processing the definition
    fn process_define(&mut self) {
        self.consume(1);
        let mut current_token = self.next().clone();
        if current_token.is_none() {
            println!("[ERROR]: Missing variable name after 'define' keyword at line {}", current_token.unwrap().line);
            exit(1);
        }

        let token = current_token.unwrap();
        if token.token_type != TokenType::VARNAME {
            println!("[ERROR]: Missing variable name after 'define' keyword at line {}", token.line);
            exit(1);
        }
        
        self.consume(1);
        
        let varname = token.value;
        let mut min = None;
        let mut max = None;
        let mut name = None;

        current_token = self.next();

        while current_token.is_some() {
            let token = current_token.unwrap();
            if token.token_type == TokenType::DECLARATION && token.value == "end" {
                break;
            }

            if token.token_type != TokenType::KEYWORD {
                println!("[ERROR]: Unexpected token '{}' at line {}", token.value, token.line);
                println!("         > Expected a keyword");
                exit(1);
            }

            if token.token_type == TokenType::KEYWORD && token.value == "min" {
                self.consume(1);
                current_token = self.next();
                if current_token.is_none() {
                    println!("[ERROR]: Missing value after 'min' keyword at line {}", token.line);
                    exit(1);
                }
                let token = current_token.unwrap();
                if token.token_type != TokenType::INTERGER && token.token_type != TokenType::FLOAT {
                    println!("[ERROR]: Unexpected token '{}' at line {}", token.value, token.line);
                    println!("         > Expected an integer or a float");
                    exit(1);
                }
                
                min = Some(token.value.parse::<f64>().unwrap());
            }

            if token.token_type == TokenType::KEYWORD && token.value == "max" {
                self.consume(1);
                current_token = self.next();
                if current_token.is_none() {
                    println!("[ERROR]: Missing value after 'max' keyword at line {}", token.line);
                    exit(1);
                }
                let token = current_token.unwrap();
                if token.token_type != TokenType::INTERGER && token.token_type != TokenType::FLOAT {
                    println!("[ERROR]: Unexpected token '{}' at line {}", token.value, token.line);
                    println!("         > Expected an integer or a float");
                    exit(1);
                }
                
                max = Some(token.value.parse::<f64>().unwrap());
            }

            if token.token_type == TokenType::KEYWORD && token.value == "name" {
                self.consume(1);
                current_token = self.next();
                if current_token.is_none() {
                    println!("[ERROR]: Missing value after 'name' keyword at line {}", token.line);
                    exit(1);
                }
                let token = current_token.unwrap();
                if token.token_type != TokenType::STRING {
                    println!("[ERROR]: Unexpected token '{}' at line {}", token.value, token.line);
                    println!("         > Expected a string");
                    exit(1);
                }
                
                name = Some(token.value);
            }

            self.consume(1);
            current_token = self.next();
        }

        if min.is_none() {
            min = Some(0.0);
        }
        if max.is_none() {
            println!("[ERROR]: Missing 'max' keyword");
            println!("         > Need to specify a maximum value for the variable '{}'", varname);
            exit(1);
        }

        let declaration = Declaration {
            varname,
            name,
            min,
            max: max.unwrap(),
        };

        self.definitions.push(declaration);
        
    }

    fn process_declaration(&mut self) {
        let declaration_name = self.next().clone().unwrap().value;
        self.consume(1);
        let mut current_token = self.next();
        if current_token.is_none() {
            println!("[ERROR]: Missing body for declaration '{}' at line {}", declaration_name, current_token.unwrap().line);
            exit(1);
        }

        if declaration_name == "root".to_string() {
            let mut _box: Option<(f64, f64, f64, f64)> = None;
            let mut color: Option<String> = None;
            let mut background: Option<String> = None;
            let mut axis: Option<(Declaration, Declaration)> = None;
            while current_token.is_some() {
                let token = current_token.unwrap();
                if token.token_type == TokenType::DECLARATION && token.value == "end" {
                    break;
                }

                if token.token_type != TokenType::KEYWORD {
                    println!("[ERROR]: Unexpected token '{}' at line {}", token.value, token.line);
                    println!("         > Expected a keyword");
                    exit(1);
                }

                if token.token_type == TokenType::KEYWORD && token.value == "box" {
                    self.consume(1);
                    let values = self.get_tuple(
                                 4, 
                        vec![TokenType::FLOAT, TokenType::INTERGER],
                        "box".to_string()
                    );

                    if values.len() != 4 {
                        println!("[ERROR]: Missing values for 'box' keyword at line {}", token.line);
                        println!("         > Need to specify 4 values");
                        exit(1);
                    }

                    _box = Some((
                        values[0].value.parse::<f64>().unwrap(),
                        values[1].value.parse::<f64>().unwrap(),
                        values[2].value.parse::<f64>().unwrap(),
                        values[3].value.parse::<f64>().unwrap(),
                    ));
                }

                if token.token_type == TokenType::KEYWORD && token.value == "color" {
                    self.consume(1);
                    current_token = self.next();
                    if current_token.is_none() {
                        println!("[ERROR]: Missing value after 'color' keyword at line {}", token.line);
                        exit(1);
                    }
                    let token = current_token.unwrap();
                    if token.token_type != TokenType::HEX {
                        println!("[ERROR]: Unexpected token '{}' at line {}", token.value, token.line);
                        println!("         > Expected a hexadecimal value");
                        exit(1);
                    }
                    
                    color = Some(token.value);
                }

                if token.token_type == TokenType::KEYWORD && token.value == "background" {
                    self.consume(1);
                    current_token = self.next();
                    if current_token.is_none() {
                        println!("[ERROR]: Missing value after 'background' keyword at line {}", token.line);
                        exit(1);
                    }
                    let token = current_token.unwrap();
                    if token.token_type != TokenType::HEX {
                        println!("[ERROR]: Unexpected token '{}' at line {}", token.value, token.line);
                        println!("         > Expected a hexadecimal value");
                        exit(1);
                    }
                    
                    background = Some(token.value);
                }

                if token.token_type == TokenType::KEYWORD && token.value == "axis" {
                    self.consume(1);
                    let values = self.get_tuple(
                                2, 
                        vec![TokenType::VAR],
                        "axis".to_string()
                    );

                    axis = Some(
                        (
                            self.get_var(values[0].clone().value).unwrap(),
                            self.get_var(values[1].clone().value).unwrap(),
                        )
                    );
                }
    
                self.consume(1);
                current_token = self.next();
            }

            if _box.is_none() {
                println!("[ERROR]: Missing 'box' keyword");
                println!("         > Need to specify a box for the root");
                exit(1);
            }
            if color.is_none() {
                color = Some("000000".to_string());
            }
            
            if background.is_none() {
                background = Some("ffffff".to_string());
            }

            if axis.is_none() {
                println!("[ERROR]: Missing 'axis' keyword");
                println!("         > Need to specify axis for the root");
                exit(1);
            }

            let root = Root {
                _box: _box.unwrap(),
                color: color.unwrap(),
                background: background.unwrap(),
                axis: axis.unwrap(),
            };

            self.root = Some(root);

        } else if declaration_name == "grid".to_string() {
            let mut color: Option<String> = None;
            let mut alpha: Option<f64> = None;
            let mut thickness: Option<f64> = None;
            let mut step: Option<f64> = None;

            while current_token.is_some() {
                let token = current_token.unwrap();
                if token.token_type == TokenType::DECLARATION && token.value == "end" {
                    break;
                }

                if token.token_type != TokenType::KEYWORD {
                    println!("[ERROR]: Unexpected token '{}' at line {}", token.value, token.line);
                    println!("         > Expected a keyword");
                    exit(1);
                }

                if token.token_type == TokenType::KEYWORD && token.value == "color" {
                    self.consume(1);
                    current_token = self.next();
                    if current_token.is_none() {
                        println!("[ERROR]: Missing value after 'color' keyword at line {}", token.line);
                        exit(1);
                    }
                    let token = current_token.unwrap();
                    if token.token_type != TokenType::HEX {
                        println!("[ERROR]: Unexpected token '{}' at line {}", token.value, token.line);
                        println!("         > Expected a hexadecimal value");
                        exit(1);
                    }
                    
                    color = Some(token.value);
                }

                if token.token_type == TokenType::KEYWORD && token.value == "alpha" {
                    self.consume(1);
                    current_token = self.next();
                    if current_token.is_none() {
                        println!("[ERROR]: Missing value after 'alpha' keyword at line {}", token.line);
                        exit(1);
                    }
                    let token = current_token.unwrap();
                    if token.token_type == TokenType::FLOAT || token.token_type == TokenType::INTERGER {
                        let alpha_value = token.value.parse::<f64>().unwrap();
                        if alpha_value > 1.0 {
                            println!("[ERROR]: Alpha value must be between 0 and 1 at line {}", token.line);
                            exit(1);
                        }
                        alpha = Some(alpha_value);
                    } else {
                        println!("[ERROR]: Unexpected token '{}' at line {}", token.value, token.line);
                        println!("         > Expected a float or an integer");
                        exit(1);
                    }
                }

                if token.token_type == TokenType::KEYWORD && token.value == "thickness" {
                    self.consume(1);
                    current_token = self.next();
                    if current_token.is_none() {
                        println!("[ERROR]: Missing value after 'thickness' keyword at line {}", token.line);
                        exit(1);
                    }
                    let token = current_token.unwrap();
                    if token.token_type == TokenType::FLOAT || token.token_type == TokenType::INTERGER {
                        thickness = Some(token.value.parse::<f64>().unwrap());
                    } else {
                        println!("[ERROR]: Unexpected token '{}' at line {}", token.value, token.line);
                        println!("         > Expected a float or an integer");
                        exit(1);
                    }
                }

                if token.token_type == TokenType::KEYWORD && token.value == "step" {
                    self.consume(1);
                    current_token = self.next();
                    if current_token.is_none() {
                        println!("[ERROR]: Missing value after 'step' keyword at line {}", token.line);
                        exit(1);
                    }
                    let token = current_token.unwrap();
                    if token.token_type == TokenType::FLOAT || token.token_type == TokenType::INTERGER {
                        let step_value = token.value.parse::<f64>().unwrap();
                        if step_value <= 0.0 {
                            println!("[ERROR]: Step value must be greater than 0 at line {}", token.line);
                            exit(1);
                        }
                        step = Some(step_value);
                    } else {
                        println!("[ERROR]: Unexpected token '{}' at line {}", token.value, token.line);
                        println!("         > Expected a float or an integer");
                        exit(1);
                    }
                }
    
                self.consume(1);
                current_token = self.next();
            }

            if color.is_none() {
                color = Some("000000".to_string());
            }

            if alpha.is_none() {
                alpha = Some(0.5);
            }

            if thickness.is_none() {
                thickness = Some(1.0);
            }

            if step.is_none() {
                step = Some(1.0);
            }

            let grid = Grid {
                color: color.unwrap(),
                alpha,
                thickness,
                step,
            };

            self.grid = Some(grid);
        } else {
            println!("[ERROR]: Unknown declaration '{}' at line {}", declaration_name, current_token.unwrap().line);
            exit(1);
        }
    }

    fn process_function(&mut self) {
        let func_name = self.next().unwrap();
        self.consume(1);
        match &func_name.value[..] {
            "line" => {
                self.process_func_line();
            },
            "graph" => {
                self.process_func_graph();
            },
            "point" => {
                self.process_func_point();
            },
            _ => {
                println!("[ERROR]: Unknown function '{}' at line {}", func_name.value, self.next().unwrap().line);
                exit(1);
            },
        }
    }

    fn get_tuple(&mut self, len: i32, allow_tokens: Vec<TokenType>, keyword_name: String) -> Vec<Token> {
        if len == 0 {
            return Vec::new();
        }
        let token_strings: Vec<String> = allow_tokens.iter().map(|x| self.lexer.get_human_readable(x.clone())).collect();
        let mut tuple: Vec<Token> = Vec::new();
        let mut current_token = self.next();
        let mut pushed = 0;
        for _ in 0..(len+ (len - 1)) {
            if current_token.is_none() {
                println!("[ERROR]: Missing values after '{keyword_name}'");
                exit(1);
            }
            let token = current_token.unwrap();
            let is_comma = token.token_type == TokenType::SYMBOL && token.value == ",";
            
            if is_comma {
                self.consume(1);
                current_token = self.next();
                continue;
            }

            if !allow_tokens.contains(&token.token_type) {
                if pushed != len {
                    println!("[ERROR]: Unexpected token '{}' at line {}", token.value, token.line);
                    println!("         > Expected one of the following: {:?}", token_strings);
                    exit(1);
                }
            } else {
                tuple.push(token.clone());
                pushed += 1;
            }

            self.consume(1);
            current_token = self.next();
        }

        self.position -= 1;

        return tuple;
    }

    /// the line function has as arguments:
    /// - from (x, y)
    /// - to (x, y)
    /// - name? "string" -> optional
    /// - color? 0x000000 -> optional
    /// - thickness? 1 -> optional
    fn process_func_line(&mut self) {
        
        let mut from: Option<(f64, f64)> = None;
        let mut to: Option<(f64, f64)> = None;
        let mut name: Option<String> = None;
        let mut color: Option<String> = None;
        let mut thickness: Option<f64> = None;

        let mut func = Function {
            name: "line".to_string(),
            args: Vec::new(),
        };

        let mut current_token = self.next();

        while current_token.is_some() {
            let token = current_token.unwrap();
            if token.token_type == TokenType::DECLARATION && token.value == "end" {
                break;
            }

            if token.token_type != TokenType::KEYWORD {
                println!("[ERROR]: Unexpected token '{}' at line {}", token.value, token.line);
                println!("         > Expected a keyword");
                exit(1);
            }

            if token.token_type == TokenType::KEYWORD && token.value == "from" {
                self.consume(1);
                let values = self.get_tuple(
                                 2, 
                        vec![TokenType::FLOAT, TokenType::INTERGER],
                        "from".to_string()
                    );

                from = Some(
                    (
                        values[0].value.parse::<f64>().unwrap(),
                        values[1].value.parse::<f64>().unwrap(),
                    )
                );
            }

            if token.token_type == TokenType::KEYWORD && token.value == "to" {
                self.consume(1);
                let values = self.get_tuple(
                                 2, 
                        vec![TokenType::FLOAT, TokenType::INTERGER],
                        "to".to_string()
                    );

                to = Some(
                    (
                        values[0].value.parse::<f64>().unwrap(),
                        values[1].value.parse::<f64>().unwrap(),
                    )
                );
            }

            if token.token_type == TokenType::KEYWORD && token.value == "name" {
                self.consume(1);
                current_token = self.next();
                if current_token.is_none() {
                    println!("[ERROR]: Missing value after 'name' keyword at line {}", token.line);
                    exit(1);
                }
                let token = current_token.unwrap();
                if token.token_type != TokenType::STRING {
                    println!("[ERROR]: Unexpected token '{}' at line {}", token.value, token.line);
                    println!("         > Expected a string");
                    exit(1);
                }
                
                name = Some(token.value);
            }

            if token.token_type == TokenType::KEYWORD && token.value == "color" {
                self.consume(1);
                current_token = self.next();
                if current_token.is_none() {
                    println!("[ERROR]: Missing value after 'color' keyword at line {}", token.line);
                    exit(1);
                }
                let token = current_token.unwrap();
                if token.token_type != TokenType::HEX {
                    println!("[ERROR]: Unexpected token '{}' at line {}", token.value, token.line);
                    println!("         > Expected a hexadecimal value");
                    exit(1);
                }
                
                color = Some(token.value);
            }

            if token.token_type == TokenType::KEYWORD && token.value == "thickness" {
                self.consume(1);
                current_token = self.next();
                if current_token.is_none() {
                    println!("[ERROR]: Missing value after 'thickness' keyword at line {}", token.line);
                    exit(1);
                }
                let token = current_token.unwrap();
                if token.token_type == TokenType::FLOAT || token.token_type == TokenType::INTERGER {
                    thickness = Some(token.value.parse::<f64>().unwrap());
                } else {
                    println!("[ERROR]: Unexpected token '{}' at line {}", token.value, token.line);
                    println!("         > Expected a float or an integer");
                    exit(1);
                }
            }

            self.consume(1);
            current_token = self.next();
        }

        if from.is_none() {
            println!("[ERROR]: Missing 'from' keyword");
            println!("         > Need to specify a starting point for the line");
            exit(1);
        } else {
            func.args.push(Arg::From(from.unwrap()));
        }

        if to.is_none() {
            println!("[ERROR]: Missing 'to' keyword");
            println!("         > Need to specify an ending point for the line");
            exit(1);
        } else {
            func.args.push(Arg::To(to.unwrap()));
        }

        if name.is_some() {
            func.args.push(Arg::Name(name.unwrap()));
        }

        if color.is_some() {
            func.args.push(Arg::Color(color.unwrap()));
        }

        if thickness.is_some() {
            func.args.push(Arg::Thickness(thickness.unwrap()));
        }

        self.functions.push(func);

    }

    /// the graph function has as arguments:
    /// - name? "string"
    /// - color? 0x000000
    /// - thickness? 1
    /// - function f(x)
    fn process_func_graph(&mut self) {
        let mut name: Option<String> = None;
        let mut color: Option<String> = None;
        let mut thickness: Option<f64> = None;
        let mut func: Option<String> = None;

        let mut function = Function {
            name: "graph".to_string(),
            args: Vec::new(),
        };

        let mut current_token = self.next();

        while current_token.is_some() {
            let token = current_token.unwrap();
            if token.token_type == TokenType::DECLARATION && token.value == "end" {
                break;
            }

            if token.token_type != TokenType::KEYWORD {
                println!("[ERROR]: Unexpected token '{}' at line {}", token.value, token.line);
                println!("         > Expected a keyword");
                exit(1);
            }

            if token.token_type == TokenType::KEYWORD && token.value == "name" {
                self.consume(1);
                current_token = self.next();
                if current_token.is_none() {
                    println!("[ERROR]: Missing value after 'name' keyword at line {}", token.line);
                    exit(1);
                }
                let token = current_token.unwrap();
                if token.token_type != TokenType::STRING {
                    println!("[ERROR]: Unexpected token '{}' at line {}", token.value, token.line);
                    println!("         > Expected a string");
                    exit(1);
                }
                
                name = Some(token.value);
            }

            if token.token_type == TokenType::KEYWORD && token.value == "color" {
                self.consume(1);
                current_token = self.next();
                if current_token.is_none() {
                    println!("[ERROR]: Missing value after 'color' keyword at line {}", token.line);
                    exit(1);
                }
                let token = current_token.unwrap();
                if token.token_type != TokenType::HEX {
                    println!("[ERROR]: Unexpected token '{}' at line {}", token.value, token.line);
                    println!("         > Expected a hexadecimal value");
                    exit(1);
                }
                
                color = Some(token.value);
            }

            if token.token_type == TokenType::KEYWORD && token.value == "thickness" {
                self.consume(1);
                current_token = self.next();
                if current_token.is_none() {
                    println!("[ERROR]: Missing value after 'thickness' keyword at line {}", token.line);
                    exit(1);
                }
                let token = current_token.unwrap();
                if token.token_type == TokenType::FLOAT || token.token_type == TokenType::INTERGER {
                    thickness = Some(token.value.parse::<f64>().unwrap());
                } else {                    
                    println!("[ERROR]: Unexpected token '{}' at line {}", token.value, token.line);
                    println!("         > Expected a float or an integer");
                    exit(1);
                }
            }

            if token.token_type == TokenType::KEYWORD && token.value == "func" {
                self.consume(1);
                current_token = self.next();
                if current_token.is_none() {
                    println!("[ERROR]: Missing value after 'function' keyword at line {}", token.line);
                    exit(1);
                }

                let token = current_token.unwrap();

                if token.token_type != TokenType::STRING {
                    println!("[ERROR]: Unexpected token '{}' at line {}", token.value, token.line);
                    println!("         > Expected a string");
                    exit(1);
                }

                func = Some(token.value);
            }

            self.consume(1);
            current_token = self.next();
        }

        if name.is_some() {
            function.args.push(Arg::Name(name.unwrap()));
        }

        if color.is_some() {
            function.args.push(Arg::Color(color.unwrap()));
        }

        if thickness.is_some() {
            function.args.push(Arg::Thickness(thickness.unwrap()));
        }

        if func.is_none() {
            println!("[ERROR]: Missing 'func' keyword");
            println!("         > Need to specify a function");
            exit(1);
        } else {
            function.args.push(Arg::Func(func.unwrap()));
        }

        self.functions.push(function);

    }

    /// the point function has as arguments:
    /// - at (x, y)
    /// - name? "string"
    /// - color? 0x000000
    fn process_func_point(&mut self) {

        let mut at: Option<(f64, f64)> = None;
        let mut name: Option<String> = None;
        let mut color: Option<String> = None;

        let mut func = Function {
            name: "point".to_string(),
            args: Vec::new(),
        };

        let mut current_token = self.next();

        while current_token.is_some() {

            let token = current_token.unwrap();
            if token.token_type == TokenType::DECLARATION && token.value == "end" {
                break;
            }

            if token.token_type != TokenType::KEYWORD {
                println!("[ERROR]: Unexpected token '{}' at line {}", token.value, token.line);
                println!("         > Expected a keyword");
                exit(1);
            }

            if token.token_type == TokenType::KEYWORD && token.value == "at" {
                self.consume(1);
                let values = self.get_tuple(
                                 2, 
                        vec![TokenType::FLOAT, TokenType::INTERGER],
                        "at".to_string()
                    );

                at = Some(
                    (
                        values[0].value.parse::<f64>().unwrap(),
                        values[1].value.parse::<f64>().unwrap(),
                    )
                );
            }

            if token.token_type == TokenType::KEYWORD && token.value == "name" {
                self.consume(1);
                current_token = self.next();
                if current_token.is_none() {
                    println!("[ERROR]: Missing value after 'name' keyword at line {}", token.line);
                    exit(1);
                }
                let token = current_token.unwrap();
                if token.token_type != TokenType::STRING {
                    println!("[ERROR]: Unexpected token '{}' at line {}", token.value, token.line);
                    println!("         > Expected a string");
                    exit(1);
                }
                
                name = Some(token.value);
            }

            if token.token_type == TokenType::KEYWORD && token.value == "color" {
                self.consume(1);
                current_token = self.next();
                if current_token.is_none() {
                    println!("[ERROR]: Missing value after 'color' keyword at line {}", token.line);
                    exit(1);
                }
                let token = current_token.unwrap();
                if token.token_type != TokenType::HEX {
                    println!("[ERROR]: Unexpected token '{}' at line {}", token.value, token.line);
                    println!("         > Expected a hexadecimal value");
                    exit(1);
                }
                
                color = Some(token.value);
            }

            self.consume(1);
            current_token = self.next();
        }

        if at.is_none() {
            println!("[ERROR]: Missing 'at' keyword");
            println!("         > Need to specify a point");
            exit(1);
        } else {
            func.args.push(Arg::At(at.unwrap()));
        }

        if name.is_some() {
            func.args.push(Arg::Name(name.unwrap()));
        }

        if color.is_some() {
            func.args.push(Arg::Color(color.unwrap()));
        }

        self.functions.push(func);

    }


    fn preprocess(&mut self) {
        self.check_declarations();
        self.check_root();
    }

    pub fn compile(&mut self) {
        self.preprocess();
        // println!("TOKENS\n{:?}\n\n", self.tokens);
        let mut current_token = self.next();
        while current_token.is_some() {
            let token = current_token.unwrap();
            match token.token_type {
                TokenType::STRING   | 
                TokenType::INTERGER | 
                TokenType::FLOAT    | 
                TokenType::HEX      | 
                TokenType::SYMBOL   | 
                TokenType::KEYWORD  | 
                TokenType::VARNAME  | 
                TokenType::VAR => {
                    println!("[ERROR]: Unexpected token '{}' at line {}", token.value, token.line);
                    println!("         > Only declarations, definitions and functions are allowed at the top level");
                    exit(1);
                },

                TokenType::DEFINE => {
                    self.process_define();
                },
                TokenType::DECLARATION => {
                    self.process_declaration();
                },
                TokenType::FUNCTION => {
                    self.process_function();
                },    
            }

            self.consume(1);
            current_token = self.next();
        }
        
        // println!("Definitions: {:?}", self.definitions);
        // println!("Root: {:?}", self.root);
        // println!("Grid: {:?}", self.grid);
        // println!("Functions: {:?}", self.functions);
        println!("{}", self.gen_svg().as_str());
    }


    //// Functions for the generation of the SVG string
    fn gen_svg(&mut self) -> String {
        let mut svg = String::new();
        svg.push_str(
            &format!(
                "<svg viewBox=\"{} {} {} {}\" xmlns=\"http://www.w3.org/2000/svg\">\n", 
                self.root.as_ref().unwrap()._box.0,
                self.root.as_ref().unwrap()._box.1,
                self.root.as_ref().unwrap()._box.2 + 10.0, 
                self.root.as_ref().unwrap()._box.3 + 10.0,
            )
        );

        svg.push_str(
            &format!(
                "<rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" fill=\"#{}\"/>\n", 
                self.root.as_ref().unwrap()._box.0,
                self.root.as_ref().unwrap()._box.1,
                self.root.as_ref().unwrap()._box.2 + 10.0, 
                self.root.as_ref().unwrap()._box.3 + 10.0,
                self.root.as_ref().unwrap().background
            )
        );

        if self.grid.is_some() {
            svg.push_str(gen_grid(
                self.grid.clone().unwrap(),
                self.root.as_ref().unwrap()._box.2 + 10.0,
                self.root.as_ref().unwrap()._box.3 + 10.0,
            ).as_str())
        }

        // root axis
        svg.push_str(draw_axis(
            self.root.clone().unwrap().axis.0,
            self.root.clone().unwrap().axis.1,
            self.root.as_ref().unwrap()._box.2,
            self.root.as_ref().unwrap()._box.3,
            self.root.as_ref().unwrap().color.clone(),
        ).as_str());

        for function in self.functions.iter() {
            let func = gen_function(
                function.clone(), 
                self.root.as_ref().unwrap()._box.2, 
                self.root.as_ref().unwrap()._box.3
            );
            svg.push_str(func.as_str());
        }

        svg.push_str("</svg>");
        svg

    }

}


fn draw_axis(
    x: Declaration,
    y: Declaration,
    w: f64,
    h: f64,
    color: String,
) -> String {
    let mut axis_string = String::new();
    
    let mut _x_min = 0.0;
    let x_max = x.max;
    let mut _y_min = 0.0;
    let y_max = y.max;

    if x.min.is_some() {
        _x_min = x.min.unwrap();
    }
    if y.min.is_some() {
        _y_min = y.min.unwrap();
    }

    // TODO: removing the min choice
    let mut from = (0.0, 0.0);
    let mut to = (x_max, 10.0);
    let thickness = 1.0;
    let color = color.clone();

    let mut func = Function {
        name: String::from("line"), 
        args: vec![
            Arg::From(from), 
            Arg::To(to), 
            Arg::Color(color.clone()), 
            Arg::Thickness(thickness)
        ]
    };

    axis_string.push_str(gen_line(&func, w, h).as_str());

    from = (0.0, 0.0);
    to = (10.0, y_max);
    func = Function {
        name: String::from("line"), 
        args: vec![
            Arg::From(from), 
            Arg::To(to), 
            Arg::Color(color), 
            Arg::Thickness(thickness)
        ]
    };

    axis_string.push_str(gen_line(&func, w, h).as_str());

    axis_string
}

fn gen_grid(grid: Grid, w: f64, h: f64) -> String {
    let mut grid_string = String::new();
    let mut alpha = 0.5;
    if grid.alpha.is_some() {
        alpha = grid.alpha.unwrap();
    }
    
    let mut thickness = 1.0;
    if grid.thickness.is_some() {
        thickness = grid.thickness.unwrap();
    }

    let mut step = 1.0;
    if grid.step.is_some() {
        step = grid.step.unwrap();
    }

    /*
    https://stackoverflow.com/questions/14208673/how-to-draw-grid-using-html5-and-canvas-or-svg
    Need to generate a pattern like this:
    <defs>
      <pattern id="grid" width="80" height="80" patternUnits="userSpaceOnUse">
        <path d="M 80 0 L 0 0 0 80" fill="none" stroke="gray" stroke-width="1"/>
      </pattern>
    </defs>
    <rect width="100%" height="100%" fill="url(#grid)" />
     */

    grid_string.push_str("<defs>\n");
    grid_string.push_str(&format!("<pattern id=\"grid\" width=\"{}\" height=\"{}\" patternUnits=\"userSpaceOnUse\">\n", step, step));
    grid_string.push_str(&format!("<path d=\"M {} 0 L 0 0 0 {}\" fill=\"none\" stroke=\"#{}\" stroke-width=\"{}\" stroke-opacity=\"{}\"/>\n", step, step, grid.color, thickness, alpha));
    grid_string.push_str("</pattern>\n");
    grid_string.push_str("</defs>\n");
    grid_string.push_str(&format!("<rect width=\"{}\" height=\"{}\" fill=\"url(#grid)\" />\n", w, h));
    
    grid_string

}

fn gen_function(func: Function, w: f64, h: f64) -> String {
    match func.name.as_ref() {
        "line" => {
            gen_line(&func, w, h)
        },
        "graph" => {
            gen_graph(&func, w, h)
        },
        "point" => {
            gen_point(&func, w, h)
        },
        _ => {
            String::new()
        },
    }    
}

#[derive(Clone, Debug)]
pub struct ArgData {
    pub from: Option<(f64, f64)>,
    pub to: Option<(f64, f64)>,
    pub at: Option<(f64, f64)>,
    pub name: Option<String>,
    pub color: Option<String>,
    pub thickness: Option<f64>,
    pub func: Option<String>,
}

fn collect_args(func: &Function) -> ArgData {
    let mut data = ArgData {
        from: None,
        to: None,
        at: None,
        name: None,
        color: None,
        thickness: None,
        func: None,
    };
    for argument in func.args.iter() {
        match argument {
            Arg::From(from) => {
                data.from = Some(from.clone());
            },
            Arg::To(to) => {
                data.to = Some(to.clone());
            },
            Arg::At(at) => {
                data.at = Some(at.clone());
            },
            Arg::Name(name) => {
                data.name = Some(name.clone());
            },
            Arg::Color(color) => {
                data.color = Some(color.clone());
            },
            Arg::Thickness(thickness) => {
                data.thickness = Some(thickness.clone());
            },
            Arg::Func(func) => {
                data.func = Some(func.clone());
            },
        }
    }
    data
}

fn gen_line(func: &Function, _w: f64, h: f64) -> String {
    let datas = collect_args(func);
    let mut from = datas.from.unwrap();
    let mut to = datas.to.unwrap();
    let name = datas.name;
    let color = datas.color;
    let thickness = datas.thickness;

    let mut line = String::new();
    // in svg the axes are inverted so we need to invert the y axis
    // and add 10 as padding
    from = (from.0 + 10.0, h - from.1);
    to = (to.0 + 10.0, h - to.1);
    line.push_str("<line ");
    line.push_str(&format!("x1=\"{}\" ", from.0));
    line.push_str(&format!("y1=\"{}\" ", from.1));
    line.push_str(&format!("x2=\"{}\" ", to.0 - 10.0));
    line.push_str(&format!("y2=\"{}\" ", to.1 + 10.0));
    line.push_str("stroke-linecap=\"round\" ");
    if name.is_some() {
        let string = name.unwrap();
        line.push_str(&format!("name=\"{}\" ", string));
    }

    if color.is_some() {
        let string = color.unwrap();
        line.push_str(&format!("stroke=\"#{}\" ", string));
    }

    if thickness.is_some() {
        let thickness = thickness.unwrap();
        line.push_str(&format!("stroke-width=\"{}\" ", thickness));
    }

    line.push_str("/>\n");
    line
}

fn gen_point(func: &Function, _w: f64, h: f64) -> String {
    let datas = collect_args(func);
    let mut at = datas.at.unwrap();
    let name = datas.name;
    let color = datas.color;

    let mut point = String::new();

    // in svg the axes are inverted so we need to invert the y axis
    // and add 10 as padding
    at = (at.0 + 10.0, h - at.1);
    point.push_str("<circle ");
    point.push_str(&format!("cx=\"{}\" ", at.0));
    point.push_str(&format!("cy=\"{}\" ", at.1));
    point.push_str("r=\"2\" ");
    if name.is_some() {
        let string = name.unwrap();
        point.push_str(&format!("name=\"{}\" ", string));
    }

    if color.is_some() {
        let string = color.unwrap();
        point.push_str(&format!("stroke=\"#{}\" ", string));
        point.push_str(&format!("fill=\"#{}\" ", string));
    } else {
        point.push_str("stroke=\"#000000\" ");
    }

    point.push_str("/>\n");

    point
}

fn gen_graph(func: &Function, w: f64, h: f64) -> String {
    let datas = collect_args(func);
    let func = datas.func.unwrap();
    let name = datas.name;
    let color = datas.color;
    let thickness = datas.thickness;

    let mut graph = String::new();

    // Will be used to generate the path
    let mut path = String::new();
    let func_string = func.clone();
    if func_string.len() == 0 {
        println!("[ERROR]: Missing function argument");
        exit(1);
    }
    for x in 0..(w as i32) {
        let x = x as f64;

        let expr = exmex::parse::<f64>(&func_string.as_str());

        match expr {
            Ok(expr) => {
                let y = expr.eval(&[(x)]);
                match y {
                    Ok(y) => {
                        if y.is_nan() {
                            continue;
                        }

                        let y = h - y;
                
                        if x == 0.0 {
                            path.push_str(&format!("M {} {} ", x + 10.0, y));
                        } else {
                            path.push_str(&format!("L {} {} ", x + 10.0, y));
                        }
                    },
                    Err(e) => {
                        println!("[ERROR]: Cannot parse function {} -> {}", func, e.msg());
                            exit(1);
                    },
                    
                }
            },
            Err(e) => {
                println!("[ERROR]: Cannot parse function {} -> {}", func, e.msg());
                exit(1);
            },
        }
    }

    graph.push_str("<path ");
    graph.push_str(&format!("d=\"{}\" ", path));
    graph.push_str("stroke-linecap=\"round\" ");
    
    if name.is_some() {
        let string = name.unwrap();
        graph.push_str(&format!("name=\"{}\" ", string));
    }

    if color.is_some() {
        let string = color.unwrap();
        graph.push_str(&format!("stroke=\"#{}\" ", string));
    } else {
        graph.push_str("stroke=\"#000000\" ");
    }

    graph.push_str("fill=\"none\" ");

    if thickness.is_some() {
        let thickness = thickness.unwrap();
        graph.push_str(&format!("stroke-width=\"{}\" ", thickness));
    }

    graph.push_str("/>\n");

    graph
}

