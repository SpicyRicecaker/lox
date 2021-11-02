use interpreter::InterpreterVisitor;
use scanner::Scanner;
use std::error::Error;
use std::io::{self, Write};

pub mod ast;
pub mod environment;
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

pub fn run(src: String, interpreter: &mut InterpreterVisitor) -> Result<(), Box<dyn Error>> {
    // println!("running");
    let mut scanner = Scanner::new(src);
    scanner.scan_tokens()?;

    let mut parser = parser::Parser::new(scanner.tokens);
    let statements = parser.parse()?;
    // dbg!(&statements);
    interpreter.interpret(statements)?;
    Ok(())
}

// Interactive
pub fn run_prompt() -> Result<(), Box<dyn Error>> {
    // create interpreter
    let mut interpreter = InterpreterVisitor::new();
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
                // TODO replace `run` w/ `run_repl`, where we only parse 1 stmt, and match that statement to see if it's an expr or a declaration
                // If it is just an expr then return it.
                // We certaintly don't want to pass a variable into run to achieve this because then it'll affect runtime performance for actual files at a nontrivial level
                let mut scanner = Scanner::new(input);
                scanner.scan_tokens()?;
                let mut parser = parser::Parser::new(scanner.tokens);
                let statements = parser.parse()?;
                match statements.get(0) {
                    Some(s) => match s {
                        ast::Stmt::Expr(e) => println!("{:#?}", interpreter.evaluate(e)?),
                        s => interpreter.execute(s)?,
                    },
                    None => eprintln!("no statements "),
                }
            }
        }
    }
    Ok(())
}

pub fn run_file(arg: &str) -> Result<(), Box<dyn Error>> {
    let content = std::fs::read_to_string(arg)?;
    let mut interpreter = InterpreterVisitor::new();
    run(content, &mut interpreter)
}
