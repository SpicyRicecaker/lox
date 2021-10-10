use ast::printer::Visitor;
use scanner::Scanner;
use std::error::Error;

pub mod ast;
pub mod error;
pub mod interpreter;
pub mod parser;
pub mod scanner;
pub mod token;

pub fn main() -> Result<(), Box<dyn Error>> {
    let mut args = std::env::args();
    args.next();

    if args.len() > 1 {
        println!("Usage: nenia [script]");
        std::process::exit(64);
    } else if args.len() != 1 {
        run_prompt()?;
    } else {
        run_file(&args.next().unwrap())?;
    }

    Ok(())
}

pub fn run(src: String) -> Result<(), Box<dyn Error>> {
    let mut scanner = Scanner::new(src);
    scanner.scan_tokens()?;

    dbg!(&scanner.tokens);

    let mut parser = parser::Parser::new(scanner.tokens);
    match parser.parse() {
        Ok(ex) => println!("{}", Visitor::new().print(ex)),
        Err(e) => eprintln!("An error occured while parsing tree: {}", e),
    };

    Ok(())
}

// Interactive
pub fn run_prompt() -> Result<(), Box<dyn Error>> {
    loop {
        let mut input = String::new();
        print!("> ");
        std::io::stdin().read_line(&mut input)?;
        match &input.trim() {
            b if b.is_empty() => {
                break;
            }
            &"exit" => {
                break;
            }
            _ => {
                run(input).expect("invalid input");
            }
        }
    }
    Ok(())
}

pub fn run_file(arg: &str) -> Result<(), Box<dyn Error>> {
    let content = std::fs::read_to_string(arg)?;
    run(content)
}
