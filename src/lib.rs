pub fn main() {
    let mut args = std::env::args();
    args.next();

    if args.len() > 1 {
        println!("Usage: lox [script]");
        std::process::exit(64);
    } else if args.len() != 1 {
        run_prompt();
    } else {
        run_file(&args.next().unwrap());
    }
}

// Interactive
pub fn run_prompt() {
    loop {
        let mut input = String::new();
        print!("> ");
        std::io::stdin().read_line(&mut input).unwrap();
        if input.is_empty() {
            break;
        } else {
            run(input);
        }
    }
}

pub fn run_file(arg: &str) {
    let content = std::fs::read_to_string(arg).unwrap();
    run(content);
}

pub fn run(src: String) {
    let tokens = src.split_whitespace();

    tokens.into_iter().for_each(|t| {
        println!("{}", t);
    });
}
