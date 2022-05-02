use std::str::Chars;

pub struct TokenValue<'a> {
    value: &'a str,
    line: usize,
}

pub enum Token<'a> {
    LeftParen(TokenValue<'a>),
    RightParen(TokenValue<'a>),
    LeftBrace(TokenValue<'a>),
    RightBrace(TokenValue<'a>),
    Comma(TokenValue<'a>),
    Dot(TokenValue<'a>),
    Minus(TokenValue<'a>),
    Plus(TokenValue<'a>),
    Semicolon(TokenValue<'a>),
    Slash(TokenValue<'a>),
    Star(TokenValue<'a>),

    Bang(TokenValue<'a>),
    BangEqual(TokenValue<'a>),
    Equal(TokenValue<'a>),
    EqualEqual(TokenValue<'a>),
    Greater(TokenValue<'a>),
    GreaterEqual(TokenValue<'a>),
    Less(TokenValue<'a>),
    LessEqual(TokenValue<'a>),

    Identifier(TokenValue<'a>),
    String(TokenValue<'a>),
    Number(TokenValue<'a>),

    And(TokenValue<'a>),
    Class(TokenValue<'a>),
    Else(TokenValue<'a>),
    False(TokenValue<'a>),
    For(TokenValue<'a>),
    Fun(TokenValue<'a>),
    If(TokenValue<'a>),
    Nil(TokenValue<'a>),
    Or(TokenValue<'a>),
    Print(TokenValue<'a>),
    Return(TokenValue<'a>),
    Super(TokenValue<'a>),
    This(TokenValue<'a>),
    True(TokenValue<'a>),
    Var(TokenValue<'a>),
    While(TokenValue<'a>),

    Error(TokenValue<'a>),
    Eof(TokenValue<'a>),
}

pub struct Scanner<'a> {
    source: &'a str,
    start: usize,
    current: usize,
    line: usize,
}

fn is_alpha(c: u8) -> bool {
    (c >= b'a' && c <= b'z') || (c >= b'A' && c <= b'Z') || c == b'_'
}

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

    pub fn scan_token(&mut self) -> Token {
        self.skip_whitespace();

        self.start = self.current;
        if self.is_at_end() {
            return Token::Eof(self.make_token());
        }

        let c = self.advance();

        if is_alpha(c) {
            return self.identifier();
        }

        if is_digit(c) {
            return self.number();
        }

        match c {
            b'(' => Token::LeftParen(self.make_token()),
            b')' => Token::RightParen(self.make_token()),
            b'{' => Token::LeftBrace(self.make_token()),
            b'}' => Token::RightBrace(self.make_token()),
            b';' => Token::Semicolon(self.make_token()),
            b',' => Token::Comma(self.make_token()),
            b'.' => Token::Dot(self.make_token()),
            b'-' => Token::Minus(self.make_token()),
            b'+' => Token::Plus(self.make_token()),
            b'/' => Token::Slash(self.make_token()),
            b'*' => Token::Star(self.make_token()),
            b'!' => {
                if self.match_(b'=') {
                    Token::BangEqual(self.make_token())
                } else {
                    Token::Bang(self.make_token())
                }
            }
            b'=' => {
                if self.match_(b'=') {
                    Token::EqualEqual(self.make_token())
                } else {
                    Token::Equal(self.make_token())
                }
            }
            b'<' => {
                if self.match_(b'=') {
                    Token::LessEqual(self.make_token())
                } else {
                    Token::Less(self.make_token())
                }
            }
            b'>' => {
                if self.match_(b'=') {
                    Token::GreaterEqual(self.make_token())
                } else {
                    Token::Greater(self.make_token())
                }
            }
            b'"' => self.string(),
            _ => Token::Error(self.error_token("Unexpected character.")),
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

    fn check_keyword(&self, rest: &str, token_type: &dyn Fn(TokenValue<'a>) -> Token) -> Token<'a> {
        if self.current - self.start == rest.len()
            && &self.source[self.start..self.current + 1] == rest
        {
            token_type(self.make_token())
        } else {
            Token::Identifier(self.make_token())
        }
    }

    fn identifier_type(&self) -> Token {
        let bytes = self.source.as_bytes();
        match bytes[self.start] {
            b'a' => self.check_keyword("nd", &Token::And),
            b'c' => self.check_keyword("lass", &Token::Class),
            b'e' => self.check_keyword("lse", &Token::Else),
            b'f' => {
                if self.current - self.start > 1 {
                    match bytes[self.start + 1] {
                        b'a' => self.check_keyword("lse", &Token::False),
                        b'o' => self.check_keyword("r", &Token::False),
                        b'u' => self.check_keyword("n", &Token::Fun),
                        _ => Token::Identifier(self.make_token()),
                    }
                } else {
                    Token::Identifier(self.make_token())
                }
            }
            b'i' => self.check_keyword("f", &Token::If),
            b'n' => self.check_keyword("il", &Token::Nil),
            b'o' => self.check_keyword("r", &Token::Or),
            b'p' => self.check_keyword("rint", &Token::Print),
            b'r' => self.check_keyword("eturn", &Token::Return),
            b's' => self.check_keyword("uper", &Token::Super),
            b't' => {
                if self.current - self.start > 1 {
                    match bytes[self.start + 1] {
                        b'h' => self.check_keyword("is", &Token::This),
                        b'r' => self.check_keyword("ue", &Token::True),
                        _ => Token::Identifier(self.make_token()),
                    }
                } else {
                    Token::Identifier(self.make_token())
                }
            }
            b'v' => self.check_keyword("ar", &Token::Var),
            b'w' => self.check_keyword("hile", &Token::While),
            _ => Token::Identifier(self.make_token()),
        }
    }

    fn identifier(&mut self) -> Token {
        while is_alpha(self.peek()) || is_digit(self.peek()) {
            self.advance();
        }
        self.identifier_type()
    }

    fn number(&mut self) -> Token {
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

        Token::Number(self.make_token())
    }

    fn string(&mut self) -> Token {
        while self.peek() != b'"' && !self.is_at_end() {
            if self.peek() == b'\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            Token::Error(self.error_token("Unterminated string."))
        } else {
            self.advance(); // the closing quote
            Token::String(self.make_token())
        }
    }

    fn peek(&self) -> u8 {
        self.source.as_bytes()[self.current]
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

    fn make_token(&self) -> TokenValue<'a> {
        TokenValue {
            value: &self.source[self.start..self.current + 1],
            line: self.line,
        }
    }

    fn error_token(&self, message: &'static str) -> TokenValue {
        TokenValue {
            value: &message,
            line: self.line,
        }
    }

    fn is_at_end(&self) -> bool {
        // Assume ASCII characters
        self.current == self.source.len()
    }
}
