use std::{collections::HashMap, fmt::Display};

pub struct Token<'a> {
    pub token_type: Type,
    pub lexeme: &'a str,
    pub line: usize,
}

impl<'a> Display for Token<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.token_type, self.lexeme, self.line)
    }
}

#[derive(Clone, Copy)]
pub enum Type {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    Identifier,
    String,
    Number,
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
    EOF,
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Type::*;
        let type_str = match *self {
            LeftParen => "left_paren",
            RightParen => "right_paren",
            LeftBrace => "left_brace",
            RightBrace => "right_brace",
            Comma => "comma",
            Dot => "dot",
            Minus => "minus",
            Plus => "plus",
            Semicolon => "semicolon",
            Slash => "slash",
            Star => "star",
            Bang => "bang",
            BangEqual => "bang_equal",
            Equal => "equal",
            EqualEqual => "equal_equal",
            Greater => "greater",
            GreaterEqual => "greater_equal",
            Less => "less",
            LessEqual => "less_equal",
            Identifier => "identifier",
            String => "string",
            Number => "number",
            And => "and",
            Class => "class",
            Else => "else",
            False => "false",
            Fun => "fun",
            For => "for",
            If => "if",
            Nil => "nil",
            Or => "or",
            Print => "print",
            Return => "return",
            Super => "super",
            This => "this",
            True => "true",
            Var => "var",
            While => "while",
            EOF => "eof",
        };
        write!(f, "{type_str}")
    }
}

pub struct Scanner<'a> {
    source: &'a [u8],
    keywords: HashMap<&'static str, Type>,
    start: usize,
    line: usize,
}

pub enum ScannerErr {
    UnexpectedCharacter(usize),
    BadUtf8(usize),
    UnterminatedString(usize),
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        let mut keywords = HashMap::new();
        keywords.insert("and", Type::And);
        keywords.insert("class", Type::Class);
        keywords.insert("else", Type::Else);
        keywords.insert("false", Type::False);
        keywords.insert("for", Type::For);
        keywords.insert("fun", Type::Fun);
        keywords.insert("if", Type::If);
        keywords.insert("nil", Type::Nil);
        keywords.insert("or", Type::Or);
        keywords.insert("print", Type::Print);
        keywords.insert("return", Type::Return);
        keywords.insert("super", Type::Super);
        keywords.insert("this", Type::This);
        keywords.insert("true", Type::True);
        keywords.insert("var", Type::Var);
        keywords.insert("while", Type::While);
        Self { source: source.as_bytes(), keywords, start: 0, line: 1 }
    }
}

impl<'a> Iterator for Scanner<'a> {
    type Item = Result<Token<'a>,ScannerErr>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut current = self.start;
        let mut t = None;
        while t.is_none() {
            self.start = current;
            if self.start >= self.source.len() { return None; }
            let byte = self.source[current];
            current += 1;
            match byte {
                b'(' => t = Some(Type::LeftParen),
                b')' => t = Some(Type::RightParen),
                b'{' => t = Some(Type::LeftBrace),
                b'}' => t = Some(Type::RightBrace),
                b',' => t = Some(Type::Comma),
                b'.' => t = Some(Type::Dot),
                b'-' => t = Some(Type::Minus),
                b'+' => t = Some(Type::Plus),
                b';' => t = Some(Type::Semicolon),
                b'*' => t = Some(Type::Star),
                b'!' => t = Some(if matches(self.source, b'=', &mut current) {
                    Type::BangEqual
                } else {
                    Type::Bang
                }),
                b'=' => t = Some(if matches(self.source, b'=', &mut current) {
                    Type::EqualEqual
                } else {
                    Type::Equal
                }),
                b'<' => t = Some(if matches(self.source, b'=', &mut current) {
                    Type::LessEqual
                } else {
                    Type::Less
                }),
                b'>' => t = Some(if matches(self.source, b'=', &mut current) {
                    Type::GreaterEqual
                } else {
                    Type::Greater
                }),
                b'/' => if matches(self.source, b'/', &mut current) {
                    while  current < self.source.len() && self.source[current] != b'\n' {
                        current += 1;
                    }
                } else {
                    t = Some(Type::Slash);
                },
                b' ' | b'\r' | b'\t' => {},
                b'\n' => self.line += 1,
                b'"' => {
                    while current < self.source.len() && self.source[current] != b'"' {
                        if self.source[current] == b'\n' { self.line += 1; }
                        current += 1;
                    }
                    if current >= self.source.len() {
                        self.start = current;
                        return Some(Err(ScannerErr::UnterminatedString(self.line)));
                    }
                    // the closing ".
                    current += 1;
                    t = Some(Type::String);
                },
                b'0'..=b'9' => {
                    while current < self.source.len() && is_digit(self.source[current]) {
                        current += 1;
                    }
                    if current + 1 < self.source.len() && self.source[current] == b'.' && is_digit(self.source[current + 1]) {
                        current += 1;
                    }
                    while current < self.source.len() && is_digit(self.source[current]) {
                        current += 1;
                    }
                    t = Some(Type::Number);
                },
                _ => {
                    if is_alpha(byte) {
                        while current < self.source.len() && (is_alpha(self.source[current]) || is_digit(self.source[current])) {
                            current += 1;
                        }
                        let keyword = std::str::from_utf8(&self.source[self.start..current]).unwrap();
                        t = match self.keywords.get(&keyword) {
                            Some(k) => Some(*k),
                            None => Some(Type::Identifier),
                        };
                    } else {
                        self.start = current;
                        return Some(Err(ScannerErr::UnexpectedCharacter(self.line)));
                    }
                }
            };
        }
        let Ok(lexeme) = std::str::from_utf8(&self.source[self.start..current]) else {
            self.start = current;
            return Some(Err(ScannerErr::BadUtf8(self.line)));
        };
        self.start = current;
        Some(Ok(Token {
            token_type: t.unwrap(),
            lexeme,
            line: self.line
        }))
    }
}

fn is_alpha(byte: u8) -> bool {
    (byte >= b'a' && byte <= b'z') || (byte >= b'A' && byte <= b'Z') || byte == b'_'
}

fn is_digit(byte: u8) -> bool {
    byte >= b'0' && byte <= b'9'
}

fn matches(source: &[u8], against: u8, current: &mut usize) -> bool {
    if *current >= source.len() {
        false
    } else if source[*current] != against {
        false
    } else {
        *current += 1;
        true
    }
}
