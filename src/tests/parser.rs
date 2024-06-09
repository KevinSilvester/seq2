use pretty_assertions::assert_eq;

use crate::{
    errors::ParserError,
    lexer::Lexer,
    parser::{Node, Parser, MAX_PAREN_DEPTH},
    tokens::Span,
};

#[test]
fn test_unexpectd_comma() {
    // comma at the start
    let input = ",1,2,3";
    let mut lexer = Lexer::new(input);
    let tokens = lexer.lex().unwrap();
    let mut parser = Parser::new(lexer.input_chars, &tokens);
    let nodes = parser.parse();

    if let Err(ParserError::UnexpectedComma(_, span)) = nodes {
        println!("{}", nodes.err().unwrap());
        assert_eq!(span.start, 1);
    } else {
        panic!();
    }

    // comma in the middle
    let input = "1,,2,3";
    let tokens = Lexer::new(input).lex().unwrap();
    let mut parser = Parser::new(input.chars().collect(), &tokens);
    let nodes = parser.parse();

    if let Err(ParserError::UnexpectedComma(_, span)) = nodes {
        println!("{}", nodes.err().unwrap());
        assert_eq!(span.start, 3);
    } else {
        panic!();
    }
}

#[test]
fn test_unexpectd_math_operator() {
    let input = "1 * 10,2,3";
    let tokens = Lexer::new(input).lex().unwrap();
    let mut parser = Parser::new(input.chars().collect(), &tokens);
    let nodes = parser.parse();

    if let Err(ParserError::UnexpectedMathOp(_, span)) = nodes {
        println!("{}", nodes.err().unwrap());
        assert_eq!(span.start, 3);
    } else {
        panic!();
    }

    let input = "1, 10,  2  ^ 10,3";
    let tokens = Lexer::new(input).lex().unwrap();
    let mut parser = Parser::new(input.chars().collect(), &tokens);
    let nodes = parser.parse();

    if let Err(ParserError::UnexpectedMathOp(_, span)) = nodes {
        println!("{}", nodes.err().unwrap());
        assert_eq!(span.start, 12);
    } else {
        panic!();
    }
}

#[test]
fn test_incomplete_number() {
    let input = "1, 10, -";
    let tokens = Lexer::new(input).lex().unwrap();
    let mut parser = Parser::new(input.chars().collect(), &tokens);
    let nodes = parser.parse();
    if let Err(ParserError::IncompleteInt(_, span)) = nodes {
        dbg!(u16::MAX);
        println!("{}", nodes.err().unwrap());
        assert_eq!(span.start, 8);
    } else {
        dbg!(&nodes);
        panic!();
    }
}

#[test]
fn test_invalid_number() {
    let input = "1, 10, (-+-),3";
    let tokens = Lexer::new(input).lex().unwrap();
    let mut parser = Parser::new(input.chars().collect(), &tokens);
    let nodes = parser.parse();
    if let Err(ParserError::InvalidInt(_, span)) = nodes {
        println!("{}", nodes.err().unwrap());
        assert_eq!(span.start, 12);
    } else {
        dbg!(&nodes);
        panic!();
    }

    let input = "1, -+%, 10, 3";
    let tokens = Lexer::new(input).lex().unwrap();
    let mut parser = Parser::new(input.chars().collect(), &tokens);
    let nodes = parser.parse();
    if let Err(ParserError::InvalidInt(_, span)) = nodes {
        println!("{}", nodes.err().unwrap());
        assert_eq!(span.start, 6);
    } else {
        dbg!(&nodes);
        panic!();
    }
}

#[test]
fn test_neg_pos_int() {
    // double minus
    let input = "--10";
    let tokens = Lexer::new(input).lex().unwrap();
    let mut parser = Parser::new(input.chars().collect(), &tokens);
    let nodes = parser.parse().unwrap();
    assert_eq!(
        nodes,
        vec![Node::Int {
            span: Span::new(1, 4),
            value: 10
        }]
    );

    // minus and plus
    let input = "-+10";
    let tokens = Lexer::new(input).lex().unwrap();
    let mut parser = Parser::new(input.chars().collect(), &tokens);
    let nodes = parser.parse().unwrap();
    assert_eq!(
        nodes,
        vec![Node::Int {
            span: Span::new(1, 4),
            value: -10
        }]
    );
}

#[test]
fn test_unmatched_paren() {
    let input = "1, (10 + 3) + (5 * 3))) , 3";
    let tokens = Lexer::new(input).lex().unwrap();
    let mut parser = Parser::new(input.chars().collect(), &tokens);
    let nodes = parser.parse();
    if let Err(ParserError::UnmatchedParen(_, span)) = nodes {
        println!("{}", nodes.err().unwrap());
        assert_eq!(span.start, 22);
    } else {
        panic!();
    }

    let input = "1, (";
    let tokens = Lexer::new(input).lex().unwrap();
    let mut parser = Parser::new(input.chars().collect(), &tokens);
    let nodes = parser.parse();
    if let Err(ParserError::UnmatchedParen(_, span)) = nodes {
        println!("{}", nodes.err().unwrap());
        assert_eq!(span.start, 4);
    } else {
        panic!();
    }
}

// #[test]
// fn test_math_expr() {
//     let input = "(1 - 5), ((10 + 3) + (5 * 3)) , 3";
//     let tokens = Lexer::new(input).lex().unwrap();
//     let mut parser = Parser::new(input.chars().collect(), &tokens);
//     let nodes = parser.parse();

//     if let Ok(nodes) = nodes {
//         assert_eq!(
//             nodes,
//             vec![
//                 Node::Int {
//                     span: Span::new(0, 7),
//                     value: -4
//                 },
//                 Node::Int {
//                     span: Span::new(9, 26),
//                     value: 28
//                 },
//                 Node::Int {
//                     span: Span::new(28, 29),
//                     value: 3
//                 }
//             ]
//         );
//     } else {
//         println!("{}", nodes.err().unwrap());
//     }
// }

#[test]
fn test_empty_maths_expr() {
    let input = "1, 2, -3, ()";
    let tokens = Lexer::new(input).lex().unwrap();
    let mut parser = Parser::new(input.chars().collect(), &tokens);
    let nodes = parser.parse();
    if let Err(ParserError::EmptyParen(_, span)) = nodes {
        println!("{}", nodes.err().unwrap());
        assert_eq!(span.start, 11);
    } else {
        panic!();
    }
}

#[test]
fn test_too_many_parenthesis() {
    let input = format!(
        "{}4+4{}, 5",
        vec!["("; MAX_PAREN_DEPTH + 2].join(""),
        vec![")"; MAX_PAREN_DEPTH + 2].join("")
    );
    let tokens = Lexer::new(&input).lex().unwrap();
    let mut parser = Parser::new(input.chars().collect(), &tokens);
    let nodes = parser.parse();
    if let Err(ParserError::TooManyParen(_, span)) = nodes {
        println!("{}", nodes.err().unwrap());
        assert_eq!(span.start, 1);
    } else {
        panic!();
    }
}
