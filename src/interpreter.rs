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
}

#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub args: Vec<Arg>,
}

#[derive(Debug, Clone)]
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
    
                self.consume(1);
                current_token = self.next();
            }

            if color.is_none() {
                color = Some("000000".to_string());
            }

            if alpha.is_none() {
                alpha = Some(0.2);
            }

            if thickness.is_none() {
                thickness = Some(1.0);
            }

            let grid = Grid {
                color: color.unwrap(),
                alpha,
                thickness,
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
        
        println!("Definitions: {:?}", self.definitions);
        println!("Root: {:?}", self.root);
        println!("Grid: {:?}", self.grid);
        println!("Functions: {:?}", self.functions);
    }


    //// Functions for the generation of the SVG string
    // fn gen_svg(&mut self) {}
    // fn compute_line(&mut self, pos_from: (f64, f64), pos_to: (f64, f64)) {}
    // fn compute_graph(&mut self, f: String) {}
    // fn compute_point(&mut self, at: (f64, f64)) {}

}
