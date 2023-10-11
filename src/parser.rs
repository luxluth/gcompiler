pub const TOP_LEVEL_DECLARATIONS: [&str; 5] = ["root", "grid", "define", "include", "end"];
pub const INNER_FUNCTIONS: [&str; 3] = ["line", "graph", "point"];
pub const KEYWORDS: [&str; 13] = [
    "min",
    "max",
    "name",
    "color",
    "background",
    "alpha",
    "thickness",
    "function",
    "from",
    "to",
    "axes",
    "box",
    "at"
];


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum TokenType {
    STRING,
    INTERGER,
    FLOAT,
    HEX,
    SYMBOL,
    KEYWORD,
    DECLARATION,
    FUNCTION,
    VARNAME,
    VAR,
    DEFINE,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Token {
    pub token_type: TokenType,
    pub value: String,
    pub line: usize,
}

pub struct Lexer {
    input: String,
    position: usize,
    line: usize,
    tokens: Vec<Token>,
    added_keywords: Vec<String>,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        Lexer {
            input,
            position: 0,
            line: 1,
            tokens: Vec::new(),
            added_keywords: Vec::new(),
        }
    }

    fn consume(&mut self, len: usize) {
        self.position += len;
    }

    fn make_token(&mut self, token_type: TokenType, value: String) {
        self.tokens.push(Token {
            token_type,
            value,
            line: self.line,
        });
    }

    fn build_string(&mut self) {
        let mut string = String::new();
        let mut is_escaped = false;
        self.consume(1);
        while self.position < self.input.len() {
            let c = self.input.chars().nth(self.position).unwrap();
            if c == '"' && !is_escaped {
                self.consume(1);
                break;
            }
            if c == '\\' && !is_escaped {
                is_escaped = true;
                self.consume(1);
                continue;
            }
            string.push(c);
            is_escaped = false;
            self.consume(1);
        }
        self.make_token(TokenType::STRING, string);
    }

    fn build_number(&mut self) -> Result<(), String> {
        let mut number = String::new();
        let mut dot_encountered = false;
        while self.position < self.input.len() {
            let c = self.input.chars().nth(self.position).unwrap();
            if c == '.' {
                if dot_encountered {
                    return Err(format!("Invalid number at line {}", self.line));
                }
                dot_encountered = true;
                number.push(c);
            } else if !c.is_ascii_digit() {
                break;
            } else {
                number.push(c);
            }

            self.consume(1);
        }

        if dot_encountered {
            self.make_token(TokenType::FLOAT, number);
            Ok(())
        } else {
            self.make_token(TokenType::INTERGER, number);
            Ok(())
        }
    }

    fn build_hex(&mut self) {
        let mut hex = String::new();
        self.consume(2);
        while self.position < self.input.len() {
            let c = self.input.chars().nth(self.position).unwrap();
            if !c.is_ascii_hexdigit() {
                break;
            }
            hex.push(c);
            self.consume(1);
        }
        self.make_token(TokenType::HEX, hex);
    }

    fn build_keyword(&mut self) -> Result<(), String>{
        let mut keyword = String::new();
        while self.position < self.input.len() {
            let c = self.input.chars().nth(self.position).unwrap();
            if !c.is_alphabetic() {
                break;
            }
            keyword.push(c);
            self.consume(1);
        }
        if self.added_keywords.contains(&keyword) {
            self.make_token(TokenType::VAR, keyword);
            return Ok(());
        }
        if KEYWORDS.contains(&keyword.as_str()) {
            self.make_token(TokenType::KEYWORD, keyword);
            return Ok(());
        } else {
            return Err(format!("Unknown keyword '{}' at line {}", keyword, self.line));
        }
    }

    fn skip_comment(&mut self) {
        self.consume(1);
        while self.position < self.input.len() {
            let c = self.input.chars().nth(self.position).unwrap();
            if c == '%' {
                self.consume(1);
                break;
            }
            self.consume(1);
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, String> {
        while self.position < self.input.len() {
            match self.input.chars().nth(self.position).unwrap() {
                '"' => {
                    self.build_string();
                },
                ' ' | '\t' => {
                    self.consume(1);
                },
                '\n' => {
                    self.consume(1);
                    self.line += 1;
                },
                '0'..='9' => {
                    // checking if it's a hex number
                    if self.input.chars().nth(self.position).unwrap() == '0' && self.input.chars().nth(self.position + 1).unwrap() == 'x' {
                        self.build_hex();
                        continue;
                    }

                    match self.build_number() {
                        Ok(_) => {},
                        Err(e) => {
                            return Err(e);
                        },
                    }   
                },

                '.' => {
                    if self.input.chars().nth(self.position + 1).unwrap().is_ascii_digit() {
                        match self.build_number() {
                            Ok(_) => {},
                            Err(e) => {
                                return Err(e);
                            },
                        }
                    } else {
                        self.consume(1);
                    }
                }

                ',' => {
                    self.make_token(TokenType::SYMBOL, self.input.chars().nth(self.position).unwrap().to_string());
                    self.consume(1);
                },
                '#' => {
                    let mut declaration = String::new();
                    self.consume(1);
                    while self.position < self.input.len() {
                        let c = self.input.chars().nth(self.position).unwrap();
                        if !c.is_alphabetic() {
                            break;
                        }
                        declaration.push(c);
                        self.consume(1);
                    }
                    if !TOP_LEVEL_DECLARATIONS.contains(&declaration.as_str()) {
                        return Err(format!("Unknown declaration '{}'", declaration));
                    }
                    let d = declaration.clone();
                    if  d == "define" {
                        self.make_token(TokenType::DEFINE, declaration);
                        self.consume(1);
                        let mut define = String::new();
                        while self.position < self.input.len() {
                            let c = self.input.chars().nth(self.position).unwrap();
                            if !c.is_alphabetic() {
                                break;
                            }
                            define.push(c);
                            self.consume(1);
                        }
                        if self.added_keywords.contains(&define) {
                            return Err(format!("Keyword '{}' already defined", define));
                        }

                        self.make_token(TokenType::VARNAME, define.clone());
                        self.added_keywords.push(define);
                    } else {
                        self.make_token(TokenType::DECLARATION, declaration);
                    }

                },
                '@' => {
                    let mut function = String::new();
                    self.consume(1);
                    while self.position < self.input.len() {
                        let c = self.input.chars().nth(self.position).unwrap();
                        if !c.is_alphabetic() {
                            break;
                        }
                        function.push(c);
                        self.consume(1);
                    }
                    if !INNER_FUNCTIONS.contains(&function.as_str()) {
                        return Err(format!("Unknown function '{}'", function));
                    }
                    self.make_token(TokenType::FUNCTION, function);
                },
                'a'..='z' | 'A'..='Z' => {
                    match self.build_keyword() {
                        Ok(_) => {},
                        Err(e) => {
                            return Err(e);
                        },
                    }
                },
                '%' => {
                    self.skip_comment();
                },
                _ => {
                    self.consume(1);
                },
                
            }
        }
        
        Ok(self.tokens.clone())
    }
}
