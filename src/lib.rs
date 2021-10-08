use scanner::Scanner;

pub mod scanner;
pub mod token;
pub mod tree;
pub mod parser;
pub struct Nenia {
    had_error: bool,
}

impl Nenia {
    pub fn new() -> Self {
        Self { had_error: false }
    }
    pub fn main(&mut self) {
        let mut args = std::env::args();
        args.next();

        if args.len() > 1 {
            println!("Usage: nenia [script]");
            std::process::exit(64);
        } else if args.len() != 1 {
            self.run_prompt();
        } else {
            self.run_file(&args.next().unwrap());
        }
    }

    // Interactive
    pub fn run_prompt(&mut self) {
        loop {
            let mut input = String::new();
            print!("> ");
            std::io::stdin().read_line(&mut input).unwrap();
            match &input.trim() {
                b if b.is_empty() => {
                    break;
                }
                &"exit" => {
                    break;
                }
                _ => {
                    self.run(input);
                    self.had_error = false;
                }
            }
        }
    }

    pub fn run_file(&self, arg: &str) {
        let content = std::fs::read_to_string(arg).unwrap();
        self.run(content);
    }

    pub fn run(&self, src: String) {
        // let tokens = src.split_whitespace();

        // tokens.into_iter().for_each(|t| {
        //     println!("{}", t);
        // });
        let mut scanner = Scanner::new(src);
        scanner.scan_tokens();

        dbg!(scanner.tokens);

        if self.had_error {
            std::process::exit(65);
        }
    }

    fn error(line: u32, msg: &str) {
        Nenia::report(line, "", msg);
    }

    /// Better would be telling them the error and where the error occured, just like Rust
    /// We all know how useless `segfault (core dumped)` is
    fn report(line: u32, location: &str, msg: &str) {
        println!("[line {}] Error{}:{}", line, location, msg);
    }
}

impl Default for Nenia {
    fn default() -> Self {
        Self::new()
    }
}
