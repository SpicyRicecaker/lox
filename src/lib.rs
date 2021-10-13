use ast::printer::Visitor;
use interpreter::Interpreter;
use scanner::Scanner;
use std::error::Error;
use std::io::{self, Write};

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

pub fn run(src: String, interpreter: &mut Interpreter) -> Result<(), Box<dyn Error>> {
    let mut scanner = Scanner::new(src);
    scanner.scan_tokens()?;

    dbg!(&scanner.tokens);

    let mut parser = parser::Parser::new(scanner.tokens);
    match parser.parse() {
        Ok(ex) => {
            println!("{}", Visitor::new().print(&ex));
            interpreter.interpret(&ex)?;
        }
        Err(e) => eprintln!("An error occured while parsing tree: {}", e),
    };

    Ok(())
}

// Interactive
pub fn run_prompt() -> Result<(), Box<dyn Error>> {
    // create interpreter
    let mut interpreter = Interpreter {};
    loop {
        let mut input = String::new();
        print!("> ");
        io::stdout().flush()?;
        io::stdin().read_line(&mut input)?;
        match &input.trim() {
            b if b.is_empty() => {
                break;
            }
            &"exit" => {
                break;
            }
            _ => {
                if let Err(e) = run(input, &mut interpreter) {
                    eprintln!("runtime error occured: {}", e);
                };
            }
        }
    }
    Ok(())
}

pub fn run_file(arg: &str) -> Result<(), Box<dyn Error>> {
    let content = std::fs::read_to_string(arg)?;
    let mut interpreter = Interpreter {};
    run(content, &mut interpreter)
}
