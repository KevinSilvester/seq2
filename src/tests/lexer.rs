use pretty_assertions::assert_eq;

use crate::{
    errors::LexicalError,
    lexer::Lexer,
    tokens::{Op, Span, Token, TokenKind},
};

#[test]
fn test_numbers() {
    let mut lexer = Lexer::new("1,2,3");
    let tokens = lexer.lex().unwrap();
    assert_eq!(
        tokens,
        vec![
            Token {
                kind: TokenKind::Int { value: 1 },
                span: Span { start: 1, end: 1 }
            },
            Token {
                kind: TokenKind::Comma,
                span: Span { start: 2, end: 2 }
            },
            Token {
                kind: TokenKind::Int { value: 2 },
                span: Span { start: 3, end: 3 }
            },
            Token {
                kind: TokenKind::Comma,
                span: Span { start: 4, end: 4 }
            },
            Token {
                kind: TokenKind::Int { value: 3 },
                span: Span { start: 5, end: 5 }
            },
        ]
    );
}

#[test]
fn test_number_too_large() {
    let mut lexer = Lexer::new("1, 2, 9_223_372_036_854_775_808");
    let tokens = lexer.lex();
    if let Err(LexicalError::NumberTooLarge(_, span)) = tokens {
        println!("{}", tokens.err().unwrap());
        assert_eq!(span, Span { start: 7, end: 31 });
    } else {
        panic!("Expected NumberTooLarge error");
    }
}

#[test]
fn test_invalid_token() {
    let mut lexer = Lexer::new("1,2,#3");
    let tokens = lexer.lex();
    if let Err(LexicalError::InvalidToken(_, span)) = tokens {
        println!("{}", tokens.err().unwrap());
        assert_eq!(span, Span { start: 5, end: 5 });
    } else {
        panic!("Expected InvalidToken error");
    }
}

#[test]
fn test_range() {
    let mut lexer = Lexer::new("{1..5}");
    let tokens = lexer.lex().unwrap();
    assert_eq!(
        tokens,
        vec![
            Token {
                kind: TokenKind::LSquiggly,
                span: Span { start: 1, end: 1 }
            },
            Token {
                kind: TokenKind::Int { value: 1 },
                span: Span { start: 2, end: 2 }
            },
            Token {
                kind: TokenKind::RngExclusive,
                span: Span { start: 3, end: 4 }
            },
            Token {
                kind: TokenKind::Int { value: 5 },
                span: Span { start: 5, end: 5 }
            },
            Token {
                kind: TokenKind::RSquiggly,
                span: Span { start: 6, end: 6 }
            },
        ]
    );

    let mut lexer = Lexer::new("{1..=5}");
    let tokens = lexer.lex().unwrap();
    assert_eq!(
        tokens,
        vec![
            Token {
                kind: TokenKind::LSquiggly,
                span: Span { start: 1, end: 1 }
            },
            Token {
                kind: TokenKind::Int { value: 1 },
                span: Span { start: 2, end: 2 }
            },
            Token {
                kind: TokenKind::RngInclusive,
                span: Span { start: 3, end: 5 }
            },
            Token {
                kind: TokenKind::Int { value: 5 },
                span: Span { start: 6, end: 6 }
            },
            Token {
                kind: TokenKind::RSquiggly,
                span: Span { start: 7, end: 7 }
            },
        ]
    );
}

#[test]
fn test_invalid_range() {
    let mut lexer = Lexer::new("{1.=.5}");
    let tokens = lexer.lex();
    if let Err(LexicalError::UnexpectedEqual(_, span)) = tokens {
        assert_eq!(span, Span { start: 3, end: 5 });
        println!("{}", tokens.err().unwrap());
    } else {
        panic!("Expected UnexpectedEqual error");
    }

    let mut lexer = Lexer::new("{1.=5}");
    let tokens = lexer.lex();
    if let Err(LexicalError::InvalidRange(_, span)) = tokens {
        assert_eq!(span, Span { start: 3, end: 4 });
        println!("{}", tokens.err().unwrap());
    } else {
        panic!("Expected InvalidRange error");
    }

    let mut lexer = Lexer::new("{1...5}");
    let tokens = lexer.lex();
    if let Err(LexicalError::InvalidRange(_, span)) = tokens {
        assert_eq!(span, Span { start: 3, end: 5 });
        println!("{}", tokens.err().unwrap());
    } else {
        panic!("Expected InvalidRange error");
    }
}

#[test]
fn test_range_arg() {
    let mut lexer = Lexer::new("{s:1,m:+20_000_000}");
    let tokens = lexer.lex().unwrap();
    assert_eq!(
        tokens,
        vec![
            Token {
                kind: TokenKind::LSquiggly,
                span: Span { start: 1, end: 1 }
            },
            Token {
                kind: TokenKind::RngStep,
                span: Span { start: 2, end: 3 }
            },
            Token {
                kind: TokenKind::Int { value: 1 },
                span: Span { start: 4, end: 4 }
            },
            Token {
                kind: TokenKind::Comma,
                span: Span { start: 5, end: 5 }
            },
            Token {
                kind: TokenKind::RngMutation,
                span: Span { start: 6, end: 7 }
            },
            Token {
                kind: TokenKind::Math(Op::Add),
                span: Span { start: 8, end: 8 }
            },
            Token {
                kind: TokenKind::Int { value: 20000000 },
                span: Span { start: 9, end: 18 }
            },
            Token {
                kind: TokenKind::RSquiggly,
                span: Span { start: 19, end: 19 }
            },
        ]
    );
}

#[test]
fn test_invalid_range_arg() {
    let mut lexer = Lexer::new("{1..=5, s2}");
    let tokens = lexer.lex();
    if let Err(LexicalError::MissingColon(_, span)) = tokens {
        assert_eq!(span, Span { start: 9, end: 9 });
        println!("{}", tokens.err().unwrap());
    } else {
        panic!("Expected MissingColon error");
    }
}

#[test]
fn test_invalid_range_syntax() {
    let mut lexer = Lexer::new("s:1");
    let tokens = lexer.lex();
    if let Err(LexicalError::MisplacedRngSyntax(_, span)) = tokens {
        println!("{}", tokens.err().unwrap());
        assert_eq!(span, Span { start: 1, end: 1 });
    } else {
        panic!("Expected MisplacedRngSyntax error");
    }

    let mut lexer = Lexer::new("1, 3, 2__000, @");
    let tokens = lexer.lex();
    if let Err(LexicalError::MisplacedRngSyntax(_, span)) = tokens {
        println!("{}", tokens.err().unwrap());
        assert_eq!(span, Span { start: 15, end: 15 });
    } else {
        panic!("Expected MisplacedRngSyntax error");
    }
}
