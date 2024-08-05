use crate::token::{Scanner, ScannerErr};

pub struct Lox {
    had_error: bool,
}

impl Lox {
    pub fn new() -> Self {
        Self { had_error: false }
    }

    pub fn run(&mut self, src: &str) {
        use ScannerErr::*;
        let mut tokens: Vec<_> = Scanner::new(src)
            .filter_map(|res| {
                match res {
                    Ok(token) => Some(token),
                    Err(err) => {
                        match err {
                            UnexpectedCharacter(line) => self.error(line, "Unexpected character."),
                            BadUtf8(line) => self.error(line, "Bad UTF-8."),
                            UnterminatedString(line) => self.error(line, "Unterminated string."),
                        };
                        None
                    },
                }
            })
            .collect();
        let line = match tokens.last() {
            Some(t) => t.line + 1,
            None => 1,
        };
        tokens.push(crate::token::Token {
            token_type: crate::token::Type::EOF,
            lexeme: "\0",
            line,
        });
        for token in tokens.into_iter() {
            println!("{token}");
        }
    }

    pub fn error(&mut self, line: usize, msg: &str) {
        self.report(line, "", msg);
    }

    fn report(&mut self, line: usize, err_where: &str, msg: &str) {
        eprintln!("[line {line}] Error{err_where}: {msg}");
        self.had_error = true;
    }

    pub fn had_error(&self) -> bool {
        self.had_error
    }

    pub fn reset_error(&mut self) {
        self.had_error = false;
    }
}
