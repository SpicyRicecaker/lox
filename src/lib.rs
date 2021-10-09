use scanner::Scanner;
use std::error::Error;

pub mod parser;
pub mod scanner;
pub mod token;
pub mod tree;
pub mod error;

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
    // let tokens = src.split_whitespace();

    // tokens.into_iter().for_each(|t| {
    //     println!("{}", t);
    // });
    let mut scanner = Scanner::new(src);
    scanner.scan_tokens()?;

    // dbg!(scanner.tokens);

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

// fn error(line: u32, msg: &str) {
//     Nenia::report(line, "", msg);
// }

// /// Better would be telling them the error and where the error occured, just like Rust
// /// We all know how useless `segfault (core dumped)` is
// fn report(line: u32, location: &str, msg: &str) {
//     println!("[line {}] Error{}:{}", line, location, msg);
// }
