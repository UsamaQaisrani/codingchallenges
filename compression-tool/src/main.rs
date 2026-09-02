use clap::Parser;
use compression_tool::decoder::Decoder;
use compression_tool::encoder::Encoder;

#[derive(Parser)]
struct Args {
    #[arg(short = 'o')]
    output_file_path: String,

    #[arg(short = 'e')]
    encode: bool,

    #[arg(short = 'd')]
    decode: bool,

    input_file_path: Option<String>,
}

fn main() {
    let args = Args::parse();

    if args.encode {
        let encoder = Encoder::new(args.input_file_path.as_deref(), args.output_file_path);
        match encoder.encode() {
            Ok(()) => println!("Encoded successfully"),
            Err(err) => {
                println!("{}", err);
                std::process::exit(1)
            }
        }
    } else if args.decode {
        let mut decoder = Decoder::new(args.input_file_path.as_deref(), args.output_file_path);
        match decoder.decode() {
            Ok(()) => println!("Decoded successfully"),
            Err(err) => {
                println!("{}", err);
                std::process::exit(1)
            }
        }
    }
}
