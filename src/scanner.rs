#[derive(Copy, Clone, PartialEq)]
#[repr(u8)]
pub enum TokenType {
    LeftParen = 0,
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
    For,
    Fun,
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
    Error,
    Eof = 39,
}

#[derive(Copy, Clone)]
pub struct Token<'a> {
    pub token_type: TokenType,
    pub value: &'a str,
    pub line: usize,
}

pub struct Scanner<'a> {
    source: &'a str,
    start: usize,
    current: usize,
    line: usize,
}

#[inline]
fn is_alpha(c: u8) -> bool {
    (c >= b'a' && c <= b'z') || (c >= b'A' && c <= b'Z') || c == b'_'
}

#[inline]
fn is_digit(c: u8) -> bool {
    c >= b'0' && c <= b'9'
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        Scanner {
            source: source,
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_token(&mut self) -> Token<'a> {
        self.skip_whitespace();

        self.start = self.current;
        if self.is_at_end() {
            return self.make_token(TokenType::Eof);
        }

        let c = self.advance();

        if is_alpha(c) {
            return self.identifier();
        }

        if is_digit(c) {
            return self.number();
        }

        match c {
            b'(' => self.make_token(TokenType::LeftParen),
            b')' => self.make_token(TokenType::RightParen),
            b'{' => self.make_token(TokenType::LeftBrace),
            b'}' => self.make_token(TokenType::RightBrace),
            b';' => self.make_token(TokenType::Semicolon),
            b',' => self.make_token(TokenType::Comma),
            b'.' => self.make_token(TokenType::Dot),
            b'-' => self.make_token(TokenType::Minus),
            b'+' => self.make_token(TokenType::Plus),
            b'/' => self.make_token(TokenType::Slash),
            b'*' => self.make_token(TokenType::Star),
            b'!' => {
                let token_type = if self.match_(b'=') {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                };
                self.make_token(token_type)
            }
            b'=' => {
                let token_type = if self.match_(b'=') {
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                };
                self.make_token(token_type)
            }
            b'<' => {
                let token_type = if self.match_(b'=') {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                };
                self.make_token(token_type)
            }
            b'>' => {
                let token_type = if self.match_(b'=') {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                };
                self.make_token(token_type)
            }
            b'"' => self.string(),
            _ => self.error_token("Unexpected character."),
        }
    }

    fn skip_whitespace(&mut self) {
        loop {
            let c = self.peek();
            match c {
                b' ' | b'\r' | b'\t' => {
                    self.advance();
                }
                b'\n' => {
                    self.line += 1;
                    self.advance();
                }
                b'/' => {
                    if self.peek_next() == b'/' {
                        while self.peek() != b'\n' && !self.is_at_end() {
                            self.advance();
                        }
                    } else {
                        return;
                    }
                }
                _ => {
                    return;
                }
            }
        }
    }

    fn check_keyword(&self, rest: &str, token_type: TokenType) -> Token<'a> {
        let token_type = if self.current - self.start == rest.len()
            && &self.source[self.start..self.current + 1] == rest
        {
            token_type
        } else {
            TokenType::Identifier
        };
        self.make_token(token_type)
    }

    fn identifier_type(&self) -> Token<'a> {
        let bytes = self.source.as_bytes();
        match bytes[self.start] {
            b'a' => self.check_keyword("nd", TokenType::And),
            b'c' => self.check_keyword("lass", TokenType::Class),
            b'e' => self.check_keyword("lse", TokenType::Else),
            b'f' => {
                if self.current - self.start > 1 {
                    match bytes[self.start + 1] {
                        b'a' => self.check_keyword("lse", TokenType::False),
                        b'o' => self.check_keyword("r", TokenType::False),
                        b'u' => self.check_keyword("n", TokenType::Fun),
                        _ => self.make_token(TokenType::Identifier),
                    }
                } else {
                    self.make_token(TokenType::Identifier)
                }
            }
            b'i' => self.check_keyword("f", TokenType::If),
            b'n' => self.check_keyword("il", TokenType::Nil),
            b'o' => self.check_keyword("r", TokenType::Or),
            b'p' => self.check_keyword("rint", TokenType::Print),
            b'r' => self.check_keyword("eturn", TokenType::Return),
            b's' => self.check_keyword("uper", TokenType::Super),
            b't' => {
                if self.current - self.start > 1 {
                    match bytes[self.start + 1] {
                        b'h' => self.check_keyword("is", TokenType::This),
                        b'r' => self.check_keyword("ue", TokenType::True),
                        _ => self.make_token(TokenType::Identifier),
                    }
                } else {
                    self.make_token(TokenType::Identifier)
                }
            }
            b'v' => self.check_keyword("ar", TokenType::Var),
            b'w' => self.check_keyword("hile", TokenType::While),
            _ => self.make_token(TokenType::Identifier),
        }
    }

    fn identifier(&mut self) -> Token<'a> {
        while is_alpha(self.peek()) || is_digit(self.peek()) {
            self.advance();
        }
        self.identifier_type()
    }

    fn number(&mut self) -> Token<'a> {
        while is_digit(self.peek()) {
            self.advance();
        }

        // Look for a fractional part
        if self.peek() == b'.' && is_digit(self.peek_next()) {
            self.advance();
            while is_digit(self.peek()) {
                self.advance();
            }
        }

        self.make_token(TokenType::Number)
    }

    fn string(&mut self) -> Token<'a> {
        while self.peek() != b'"' && !self.is_at_end() {
            if self.peek() == b'\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            self.error_token("Unterminated string.")
        } else {
            self.advance(); // the closing quote
            self.make_token(TokenType::String)
        }
    }

    fn peek(&self) -> u8 {
        if self.is_at_end() {
            b'\0'
        } else {
            self.source.as_bytes()[self.current]
        }
    }

    fn peek_next(&self) -> u8 {
        if self.is_at_end() {
            b'\0'
        } else {
            self.source.as_bytes()[self.current + 1]
        }
    }

    fn match_(&mut self, expected: u8) -> bool {
        if self.is_at_end() || self.source.as_bytes()[self.current] != expected {
            false
        } else {
            self.current += 1;
            true
        }
    }

    fn advance(&mut self) -> u8 {
        // Assume ASCII characters
        self.current += 1;
        self.source.as_bytes()[self.current - 1]
    }

    fn make_token(&self, token_type: TokenType) -> Token<'a> {
        Token {
            token_type: token_type,
            value: &self.source[self.start..self.current],
            line: self.line,
        }
    }

    fn error_token(&self, message: &'static str) -> Token<'a> {
        Token {
            token_type: TokenType::Error,
            value: &message,
            line: self.line,
        }
    }

    #[inline]
    fn is_at_end(&self) -> bool {
        // Assume ASCII characters
        self.current == self.source.len()
    }
}
