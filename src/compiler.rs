use crate::chunk::{Chunk, OpCode};
use crate::debug::disassemble_chunk;
use crate::scanner::{Scanner, Token, TokenType};
use crate::value::Value;
use num_enum::TryFromPrimitive;

pub struct Parser<'a> {
    scanner: Scanner<'a>,
    current: Token<'a>,
    previous: Token<'a>,
    had_error: bool,
    panic_mode: bool,
    chunk: &'a mut Chunk,
}

#[derive(Copy, Clone, PartialEq, PartialOrd, TryFromPrimitive)]
#[repr(u8)]
enum Precedence {
    None = 0,
    Assignment, // =
    Or,         // or
    And,        // and
    Equality,   // == !=
    Comparison, // < > >= <=
    Term,       // + -
    Factor,     // * /
    Unary,      // ! -
    Call,       // . ()
    Primary,
}

type ParseFn = fn(&mut Parser);

struct ParseRule {
    prefix: Option<ParseFn>,
    infix: Option<ParseFn>,
    precedence: Precedence,
}

const RULES: [ParseRule; 40] = [
    // [0] LeftParen
    ParseRule {
        prefix: Some(|p| Parser::grouping(p)),
        infix: None,
        precedence: Precedence::None,
    },
    // [1] RightParen
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    // [2] LeftBrace
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    // [3] RightBrace
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    // [4] Comma
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    // [5] Dot
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    // [6] Minus
    ParseRule {
        prefix: Some(|p| Parser::unary(p)),
        infix: Some(|p| Parser::binary(p)),
        precedence: Precedence::Term,
    },
    // [7] Plus
    ParseRule {
        prefix: None,
        infix: Some(|p| Parser::binary(p)),
        precedence: Precedence::Term,
    },
    // [8] Semicolon
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    // [9] Slash
    ParseRule {
        prefix: None,
        infix: Some(|p| Parser::binary(p)),
        precedence: Precedence::Factor,
    },
    // [10] Star
    ParseRule {
        prefix: None,
        infix: Some(|p| Parser::binary(p)),
        precedence: Precedence::Factor,
    },
    // [11] Bang
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    // [12] BangEqual
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    // [13] Equal
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    // [14] EqualEqual
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    // [15] Greater
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    // [16] GreaterEqual
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    // [17] Less
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    // [18] LessEqual
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    // [19] Identifier
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    // [20] String
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    // [21] Number
    ParseRule {
        prefix: Some(|p| Parser::number(p)),
        infix: None,
        precedence: Precedence::None,
    },
    // [22] And
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    // [23] Class
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    // [24] Else
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    // [25] False
    ParseRule {
        prefix: Some(|p| Parser::literal(p)),
        infix: None,
        precedence: Precedence::None,
    },
    // [26] For
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    // [27] Fun
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    // [28] If
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    // [29] Nil
    ParseRule {
        prefix: Some(|p| Parser::literal(p)),
        infix: None,
        precedence: Precedence::None,
    },
    // [30] Or
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    // [31] Print
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    // [32] Return
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    // [33] Super
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    // [34] This
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    // [35] True
    ParseRule {
        prefix: Some(|p| Parser::literal(p)),
        infix: None,
        precedence: Precedence::None,
    },
    // [36] Var
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    // [37] While
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    // [38] Error
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    // [39] Eof
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
];

fn get_rule<'a>(token_type: TokenType) -> &'static ParseRule {
    &RULES[token_type as usize]
}

impl<'a> Parser<'a> {
    pub fn new(source: &'a str, chunk: &'a mut Chunk) -> Self {
        let default_token = Token {
            token_type: TokenType::Eof,
            value: "",
            line: 0,
        };
        Parser {
            scanner: Scanner::new(source),
            current: default_token,
            previous: default_token,
            had_error: false,
            panic_mode: false,
            chunk: chunk,
        }
    }

    pub fn compile(&mut self) -> bool {
        self.advance();
        self.expression();
        self.consume(TokenType::Eof, "Expected end of expression.");
        self.end_compiler();
        !self.had_error
    }

    fn advance(&mut self) {
        self.previous = self.current;

        loop {
            self.current = self.scanner.scan_token();

            // Skip until we get a non-error token
            match self.current.token_type {
                TokenType::Error => self.error_at_current(self.current.value),
                _ => break,
            }
        }
    }

    fn consume(&mut self, token_type: TokenType, message: &str) {
        if self.current.token_type == token_type {
            self.advance();
        } else {
            self.error_at_current(message);
        }
    }

    fn emit_byte(&mut self, byte: u8) {
        self.chunk.write_chunk(byte, self.previous.line);
    }

    fn emit_bytes(&mut self, byte1: u8, byte2: u8) {
        self.emit_byte(byte1);
        self.emit_byte(byte2);
    }

    fn emit_return(&mut self) {
        self.emit_byte(OpCode::Return as u8);
    }

    fn make_constant(&mut self, value: Value) -> u8 {
        let constant = self.chunk.add_constant(value);
        constant as u8
    }

    fn emit_constant(&mut self, value: Value) {
        let constant = self.make_constant(value);
        self.emit_bytes(OpCode::Constant as u8, constant);
    }

    fn end_compiler(&mut self) {
        self.emit_return();

        if cfg!(feature = "debug-print-code") {
            if !self.had_error {
                disassemble_chunk(&self.chunk, "code");
            }
        }
    }

    fn binary(&mut self) {
        let operator_type = self.previous.token_type;
        let rule = get_rule(operator_type);
        self.parse_precedence(Precedence::try_from(rule.precedence as u8 + 1).unwrap());
        match operator_type {
            TokenType::Plus => self.emit_byte(OpCode::Add as u8),
            TokenType::Minus => self.emit_byte(OpCode::Subtract as u8),
            TokenType::Star => self.emit_byte(OpCode::Multiply as u8),
            TokenType::Slash => self.emit_byte(OpCode::Divide as u8),
            _ => (), // unreachable
        }
    }

    fn literal(&mut self) {
        match self.previous.token_type {
            TokenType::False => self.emit_byte(OpCode::False as u8),
            TokenType::Nil => self.emit_byte(OpCode::Nil as u8),
            TokenType::True => self.emit_byte(OpCode::True as u8),
            _ => (), // unreachable
        }
    }

    fn grouping(&mut self) {
        self.expression();
        self.consume(TokenType::RightParen, "Expect ')' after expression.");
    }

    fn number(&mut self) {
        let value: f64 = self.previous.value.parse().unwrap();
        self.emit_constant(Value::number(value));
    }

    fn unary(&mut self) {
        let operator_type = self.previous.token_type;
        // Compile the operand
        self.parse_precedence(Precedence::Unary);

        // Emit the operator instruction
        match operator_type {
            TokenType::Minus => self.emit_byte(OpCode::Negate as u8),
            _ => (), // unreachable
        }
    }

    fn parse_precedence(&mut self, precedence: Precedence) {
        self.advance();
        let prefix_rule = get_rule(self.previous.token_type).prefix;
        match prefix_rule {
            None => self.error("Expect expression."),
            Some(prefix_rule) => {
                prefix_rule(self);

                while precedence <= get_rule(self.current.token_type).precedence {
                    self.advance();
                    let infix_rule = get_rule(self.previous.token_type).infix;
                    match infix_rule {
                        None => self.error("Expect expression."),
                        Some(infix_rule) => infix_rule(self),
                    }
                }
            }
        }
    }

    fn expression(&mut self) {
        self.parse_precedence(Precedence::Assignment);
    }

    fn error_at_current(&mut self, message: &str) {
        self.error_at(self.current, message);
    }

    fn error_at(&mut self, token: Token, message: &str) {
        if self.panic_mode {
            return;
        }
        self.panic_mode = true;

        eprint!("[line {}] Error", token.line);

        match token.token_type {
            TokenType::Eof => eprint!(" at end"),
            TokenType::Error => (),
            _ => eprint!(" at '{}'", token.value),
        }

        eprintln!(": {}", message);
        self.had_error = true;
    }

    fn error(&mut self, message: &str) {
        self.error_at(self.current, message);
    }
}
