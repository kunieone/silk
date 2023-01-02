use std::{env, fs, io::Read};

use token::{Symbol, Token, TokenType};

pub mod lex;
pub mod pos;
pub mod token;

// fn _test() {
//     lex::lex(input);
// }
#[derive(Debug, PartialEq, Clone)]
pub struct TokensBox(Vec<Token>);
impl TokensBox {
    pub fn to_string(&self) -> String {
        let mut strs = "".to_string();
        for e in &self.0 {
            if e.tp == TokenType::Symbol(Symbol::Crge) {
                strs += &"\n"
            } else {
                strs += &format!("{} ", e)
            }
        }
        strs
    }
    pub fn to_vec(&self) -> Vec<Token> {
        self.0.clone()
    }
    pub fn debug(&self) {
        for e in &self.0 {
            if e.tp == TokenType::Symbol(Symbol::Crge) {
                println!()
            } else {
                print!("{} ", e.tp)
            }
        }
    }
}
fn main() {
    let filename = read_args().unwrap();
    let silk_text = read_file(&filename);
    let tokens_box = TokensBox(lex::lex(&silk_text));
    tokens_box.debug();
}

fn read_args() -> Option<String> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Error: expected at least 2 arguments");
        return None;
    }

    Some(args[1].clone())
}

fn read_file(filename: &str) -> String {
    let mut file = match fs::File::open(filename) {
        Ok(file) => file,
        Err(err) => {
            panic!("Error opening file: {}", err);
        }
    };

    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Ok(_) => contents,
        Err(err) => panic!("Error reading file: {}", err),
    }
}
