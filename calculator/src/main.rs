use calculator::calculator::calculate;
use clap::Parser;

#[derive(Parser)]
struct Args {
    exp: String,
}

fn main() {
    let args = Args::parse();
    match calculate(&args.exp) {
        Ok(res) => println!("{} = {}", args.exp, res),
        Err(e) => eprintln!("{}", e),
    }
}
