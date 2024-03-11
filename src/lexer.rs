use std::{iter::Peekable, str::Chars};

use crate::{errors::LexicalError, tokens::Token};

type LexResult = Result<Vec<Token>, LexicalError>;
type TokenResult = Result<Token, LexicalError>;

#[derive(Debug)]
pub struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
    position: u16,
    ch: char,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input: input.chars().peekable(),
            position: 1,
            ch: '\0',
        }
    }

    fn next_char(&mut self) {
        self.input.next();
        self.position += 1;
    }

    pub fn lex(&mut self) -> LexResult {
        let mut tokens = vec![];

        while let Some(ch) = self.input.peek() {
            self.ch = *ch;

            match *ch {
                ' ' => {
                    tokens.push(Token::Space { pos: self.position });
                    self.next_char();
                }
                ',' => {
                    tokens.push(Token::Comma { pos: self.position });
                    self.next_char();
                }
                '0'..='9' => {
                    let number = self.tokenize_numbers()?;
                    tokens.push(number);
                }
                '.' => {
                    let range = self.tokenize_range()?;
                    tokens.push(range);
                }
                's' | 'm' => {
                    let range_arg = self.tokenize_range_arg()?;
                    tokens.push(range_arg);
                }
                '+' | '-' | '*' | '/' | '^' => {
                    let operator = self.tokenize_operator();
                    tokens.push(operator);
                }
                '(' | ')' | '{' | '}' => {
                    let paren = self.tokenize_parenteses();
                    tokens.push(paren);
                }
                _ => {
                    return Err(LexicalError::InvalidToken(self.ch, self.position));
                }
            }
        }

        Ok(tokens)
    }

    fn tokenize_parenteses(&mut self) -> Token {
        let paren = match self.ch {
            '(' => Token::LParen { pos: self.position },
            ')' => Token::RParen { pos: self.position },
            '{' => Token::LSquiggly { pos: self.position },
            '}' => Token::RSquiggly { pos: self.position },
            _ => unreachable!(),
        };
        self.next_char();
        paren
    }

    fn tokenize_operator(&mut self) -> Token {
        let op = match self.ch {
            '+' => Token::MathAdd { pos: self.position },
            '-' => Token::MathSub { pos: self.position },
            '*' => Token::MathMul { pos: self.position },
            '/' => Token::MathDiv { pos: self.position },
            '^' => Token::MathPow { pos: self.position },
            _ => unreachable!(),
        };
        self.next_char();
        op
    }

    fn tokenize_range(&mut self) -> TokenResult {
        let mut dot_count = 0;
        let mut inclusive = false;
        let start_pos = self.position;
        let mut prev_ch = self.ch;

        while let Some(ch @ ('.' | '=')) = self.input.peek() {
            match *ch {
                '.' => {
                    if prev_ch == '=' {
                        return Err(LexicalError::UnexpectedEqual(start_pos));
                    }

                    dot_count += 1;

                    prev_ch = *ch;
                    self.next_char();
                }
                '=' => {
                    inclusive = true;
                    prev_ch = *ch;
                    self.next_char();
                }
                _ => {}
            }
        }

        if dot_count != 2 {
            return Err(LexicalError::InvalidRange(start_pos, self.position - 1));
        }

        if inclusive {
            Ok(Token::RngInclusive {
                start_pos,
                end_pos: self.position - 1,
            })
        } else {
            Ok(Token::RngExclusive {
                start_pos,
                end_pos: self.position - 1,
            })
        }
    }

    fn tokenize_range_arg(&mut self) -> TokenResult {
        let start_pos = self.position;
        self.next_char();

        if let Some(':') = self.input.peek() {
            match self.ch {
                's' => {
                    self.next_char();
                    Ok(Token::RngStep {
                        start_pos,
                        end_pos: self.position - 1,
                    })
                }
                'm' => {
                    self.next_char();
                    Ok(Token::RngMutation {
                        start_pos,
                        end_pos: self.position - 1,
                    })
                }
                _ => unreachable!(),
            }
        } else {
            Err(LexicalError::MissingColon(self.ch, self.position - 1))
        }
    }

    fn tokenize_numbers(&mut self) -> TokenResult {
        let mut number = String::new();
        let mut str_val = String::new();
        let start_pos = self.position;

        while let Some(ch @ ('0'..='9' | '_')) = self.input.peek() {
            if *ch != '_' {
                number.push(*ch);
            }
            str_val.push(*ch);
            self.next_char();
        }

        match number.parse::<u32>() {
            Ok(val) => Ok(Token::Int {
                val,
                str_val,
                start_pos,
                end_pos: self.position - 1,
            }),
            Err(_) => Err(LexicalError::MalformedNumber(
                number,
                start_pos,
                self.position - 1,
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_numbers() {
        let mut lexer = Lexer::new("1,2,3");
        let tokens = lexer.lex().unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Int {
                    start_pos: 1,
                    end_pos: 1,
                    val: 1,
                    str_val: "1".to_string()
                },
                Token::Comma { pos: 2 },
                Token::Int {
                    start_pos: 3,
                    end_pos: 3,
                    val: 2,
                    str_val: "2".to_string()
                },
                Token::Comma { pos: 4 },
                Token::Int {
                    start_pos: 5,
                    end_pos: 5,
                    val: 3,
                    str_val: "3".to_string()
                },
            ]
        );
    }

    #[test]
    fn test_operator() {
        let mut lexer = Lexer::new("+,-,*,/,^");
        let tokens = lexer.lex().unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::MathAdd { pos: 1 },
                Token::Comma { pos: 2 },
                Token::MathSub { pos: 3 },
                Token::Comma { pos: 4 },
                Token::MathMul { pos: 5 },
                Token::Comma { pos: 6 },
                Token::MathDiv { pos: 7 },
                Token::Comma { pos: 8 },
                Token::MathPow { pos: 9 },
            ]
        );
    }

    #[test]
    fn test_range() {
        let mut lexer = Lexer::new("{1..=5}");
        let tokens = lexer.lex().unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::LSquiggly { pos: 1 },
                Token::Int {
                    start_pos: 2,
                    end_pos: 2,
                    val: 1,
                    str_val: "1".to_string()
                },
                Token::RngInclusive {
                    start_pos: 3,
                    end_pos: 5
                },
                Token::Int {
                    start_pos: 6,
                    end_pos: 6,
                    val: 5,
                    str_val: "5".to_string()
                },
                Token::RSquiggly { pos: 7 },
            ]
        );
    }

    #[test]
    fn test_invalid_range() {
        let mut lexer = Lexer::new("{1.=.5}");
        let tokens = lexer.lex();
        if let Err(LexicalError::UnexpectedEqual(position)) = tokens {
            assert_eq!(position, 3);
            println!("{}", tokens.err().unwrap());
        } else {
            panic!("Expected InvalidRange error");
        }
    }

    #[test]
    fn test_invalid_range2() {
        let mut lexer = Lexer::new("{1...5}");
        let tokens = lexer.lex();
        if let Err(LexicalError::InvalidRange(start, end)) = tokens {
            println!("{}", tokens.err().unwrap());
            assert_eq!(start, 3);
            assert_eq!(end, 5);
        } else {
            panic!("Expected InvalidRange error");
        }
    }

    #[test]
    fn test_invalid_range3() {
        let mut lexer = Lexer::new("{1.=5}");
        let tokens = lexer.lex();
        if let Err(LexicalError::InvalidRange(start, end)) = tokens {
            println!("{}", tokens.err().unwrap());
            assert_eq!(start, 3);
            assert_eq!(end, 4);
        } else {
            panic!("Expected InvalidRange error");
        }
    }

    #[test]
    fn test_lex_range_step() {
        let mut lexer = Lexer::new("{1..=5, s:2}");
        let tokens = lexer.lex().unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::LSquiggly { pos: 1 },
                Token::Int {
                    start_pos: 2,
                    end_pos: 2,
                    val: 1,
                    str_val: "1".to_string()
                },
                Token::RngInclusive {
                    start_pos: 3,
                    end_pos: 5
                },
                Token::Int {
                    start_pos: 6,
                    end_pos: 6,
                    val: 5,
                    str_val: "5".to_string()
                },
                Token::Comma { pos: 7 },
                Token::Space { pos: 8 },
                Token::RngStep {
                    start_pos: 9,
                    end_pos: 10
                },
                Token::Int {
                    start_pos: 11,
                    end_pos: 11,
                    val: 2,
                    str_val: "2".to_string()
                },
                Token::RSquiggly { pos: 12 }
            ]
        );
    }

    #[test]
    fn test_lex_range_mutation() {
        let mut lexer = Lexer::new("{1..=5, m:+20_500_500}");
        let tokens = lexer.lex().unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::LSquiggly { pos: 1 },
                Token::Int {
                    start_pos: 2,
                    end_pos: 2,
                    val: 1,
                    str_val: "1".to_string()
                },
                Token::RngInclusive {
                    start_pos: 3,
                    end_pos: 5
                },
                Token::Int {
                    start_pos: 6,
                    end_pos: 6,
                    val: 5,
                    str_val: "5".to_string()
                },
                Token::Comma { pos: 7 },
                Token::Space { pos: 8 },
                Token::RngMutation {
                    start_pos: 9,
                    end_pos: 10
                },
                Token::MathAdd { pos: 11 },
                Token::Int {
                    start_pos: 12,
                    end_pos: 21,
                    val: 20_500_500,
                    str_val: "20_500_500".to_string()
                },
                Token::RSquiggly { pos: 22 }
            ]
        );
    }

    #[test]
    fn test_lex_range_mutation2() {
        let mut lexer = Lexer::new("{1..=5, m:2, s:-10}");
        let tokens = lexer.lex().unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::LSquiggly { pos: 1 },
                Token::Int {
                    start_pos: 2,
                    end_pos: 2,
                    val: 1,
                    str_val: "1".to_string()
                },
                Token::RngInclusive {
                    start_pos: 3,
                    end_pos: 5
                },
                Token::Int {
                    start_pos: 6,
                    end_pos: 6,
                    val: 5,
                    str_val: "5".to_string()
                },
                Token::Comma { pos: 7 },
                Token::Space { pos: 8 },
                Token::RngMutation {
                    start_pos: 9,
                    end_pos: 10
                },
                Token::Int {
                    start_pos: 11,
                    end_pos: 11,
                    val: 2,
                    str_val: "2".to_string()
                },
                Token::Comma { pos: 12 },
                Token::Space { pos: 13 },
                Token::RngStep {
                    start_pos: 14,
                    end_pos: 15
                },
                Token::MathSub { pos: 16 },
                Token::Int {
                    start_pos: 17,
                    end_pos: 18,
                    val: 10,
                    str_val: "10".to_string()
                },
                Token::RSquiggly { pos: 19 }
            ]
        );
    }

    #[test]
    fn test_invalid_range_arg() {
        let mut lexer = Lexer::new("{1..=5, s}");
        let tokens = lexer.lex();
        if let Err(LexicalError::MissingColon(ch, position)) = tokens {
            println!("{}", tokens.err().unwrap());
            assert_eq!(ch, 's');
            assert_eq!(position, 9);
        } else {
            panic!("Expected MissingColon error");
        }
    }
}
