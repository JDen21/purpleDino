use std::char;

/**
    TERMINALS:
        OPEN_PAREN
        CLOSE_PAREN
        ADD
        MINUS
        MULTIPLY
        DIVIDE
        MODULO
        DEC
        INT
        STR
        CHAR
        VAR
        END_STMT

    NON-TERMINALS:
        <PROGRAM>
        <STMT>
        <STR_CONCAT>
        <ARITHMETIC>
        <FACTOR>
        <TERM>
        <ARITHMETIC_OPERATORS>
        <ARITHMETIC_OPERANDS>

    PRODUCTION RULES:
        <PROGRAM>
            ::= <STMT>
        <STMT>
            ::= <ARITHMETIC> END_STMT
            | <STR_CONCAT> END_STMT
        <STR_CONCAT>
            ::= STR ADD <STR_CONCAT> | STR
        <ARITHMETIC>
            ::= OPEN_PAREN <ARITHMETIC_OPERANDS> CLOSE_PAREN
            | <ARITHMETIC_OPERANDS> <ARITHMETIC_OPERATORS> <ARITHMETIC_OPERANDS>
            | <ARITHMETIC_OPERANDS>
        <ARITHMETIC_OPERANDS>
            ::= VAR
            | DEC
            | INT
            | <ARITHMETIC>
        <ARITHMETIC_OPERATORS>
            ::= ADD
            | MINUS
            | MULTIPLY
            | DIVIDE
            | MODULO
*/

#[derive(Debug)]
pub enum Token {
    OpenParen,
    CloseParen,
    Add,
    Minus,
    Multiply,
    Divide,
    Modulo,
    Dec(f64),
    Int(i64),
    Str(String),
    Char(char),
    Var(String),
    EndStmt,
    None,
}

pub fn tokenizer(program: &str) -> Vec<Token> {
    let mut token_list = vec![];
    let mut curr_tok = Token::None;
    let mut chars = program.chars();

    while let Some(nxt_char) = chars.next() {
        match nxt_char {
            // * tmp
            ';' => {
                token_list.push(curr_tok);
                curr_tok = Token::None;
            }
            '0'..='9' => match curr_tok {
                // [ (+-/*% ]0..9
                Token::OpenParen
                | Token::Add
                | Token::Minus
                | Token:: Divide
                | Token::Multiply
                | Token::Modulo
                | Token::EndStmt => {
                    token_list.push(curr_tok);
                    let converted = nxt_char.to_string().parse::<i64>()
                        .expect("Unparseable char");
                    curr_tok = Token::Int(converted);
                }
                // [0.0..9.0] 0..9
                Token::Dec(curr_value) => {
                    let converted = nxt_char.to_string().parse::<f64>()
                        .expect("Unparseable char");
                    let updated_val = (curr_value * 10.0) + converted;
                    curr_tok = Token::Dec(updated_val);
                }
                // [0..9] 0..9
                Token::Int(curr_value) => {
                    let converted = nxt_char.to_string().parse::<i64>()
                        .expect("Unparseable char");
                    let updated_val = (curr_value * 10) + converted;
                    curr_tok = Token::Int(updated_val);
                }
                // [str | id] 0..9
                Token::Str(ref mut curr_value)
                | Token::Var(ref mut curr_value)=> {
                    curr_value.push(nxt_char);
                }
                Token::Char(curr_value) => {
                    if curr_value.len_utf8() > 0 {
                        panic!("Invalid size character{}", curr_value);
                    }
                    curr_tok = Token::Char(nxt_char);
                }
                // 0..9
                Token::None => {
                    let converted = nxt_char.to_string().parse::<i64>()
                        .expect("Unparseable char");
                    curr_tok = Token::Int(converted);
                }
                _ => { panic!("Invalid token in place {:#?}.", curr_tok) }
            },
            _ => {
                panic!("Unhandled character {}", nxt_char);
            }
        }
    }

    match curr_tok {
        Token::None => {}
        _ => panic!("Invalid program terminator.")
    }

    token_list.push(curr_tok);
    token_list
}

#[cfg(test)]
mod ast_test {
    use crate::ast::{tokenizer, Token};

    #[test]
    #[should_panic]
    fn it_terminates_program () {
        let line_stmt = "12";
        tokenizer(line_stmt);
    }

    #[test]
    fn it_test_possible_paths_for_numeric_chars () {
        let mut program = "12;";
        let mut tokens = tokenizer(program);
        assert_eq!(2, tokens.len(), "an int literal is a valid program");

        let mut expected_int_val = match tokens.get(0).unwrap() {
            Token::Int(data) => data,
            _ => panic!("Unexpected not number type.")
        };
        assert_eq!(12, *expected_int_val, "an int literal is stored properly");

        program = ";12;";
        tokens = tokenizer(program);
        assert_eq!(3, tokens.len(), "an int literal is a valid program statement");

        expected_int_val = match tokens.get(1).unwrap() {
            Token::Int(data) => data,
            _ => panic!("Unexpected not number type.")
        };
        assert_eq!(12, *expected_int_val, "an int literal statement is stored properly");
    }
}
