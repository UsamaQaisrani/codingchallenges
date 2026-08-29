use clap::Parser;
use compression_tool::encoder::Encoder;

#[derive(Parser)]
struct Args {
    #[arg(short = 'o')]
    output_file_path: String,

    input_file_path: Option<String>,
}

fn main() {
    let args = Args::parse();
    let encoder = Encoder::new(args.input_file_path.as_deref(), args.output_file_path);

    match encoder.encode() {
        Ok(()) => println!("Encoded successfully"),
        Err(err) => {
            println!("{}", err);
            std::process::exit(1)
        }
    }
}
