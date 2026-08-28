use clap::Parser;
use common::input_reader::read_string;
use json_parser::{lexer::Lexer, parser::Parser as JsonParser};

#[derive(Parser)]
struct Args {
    file: Option<String>,
}

fn main() {
    let args = Args::parse();
    let input = read_string(args.file.as_deref());
    match input {
        Ok(input) => {
            let mut lexer = Lexer::new(&input);
            let tokens = lexer.process_tokens();
            match tokens {
                Ok(tokens) => {
                    let mut parser = JsonParser::new(tokens);
                    let output = parser.parse();
                    match output {
                        Ok(output) => {
                            println!("Valid JSON: \n{}", output);
                            std::process::exit(0)
                        }
                        Err(err) => {
                            println!("{}", err);
                            std::process::exit(1)
                        }
                    }
                }
                Err(err) => {
                    println!("{}", err);
                    std::process::exit(1)
                }
            }
        }
        Err(err) => {
            println!("{}", err);
            std::process::exit(1)
        }
    }
}
