use clap::Parser;

#[derive(Parser)]
struct Args {
    expr: String,
}

fn main() {
    let _args = Args::parse();
}
