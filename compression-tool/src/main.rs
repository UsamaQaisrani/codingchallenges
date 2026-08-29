use clap::Parser;
use compression_tool::encoder::Encoder;

#[derive(Parser)]
struct Args {
    file_path: Option<String>,
}

fn main() {
    let args = Args::parse();
    let encoder = Encoder {};
    let _res = encoder.encode(args.file_path.as_deref());
}
