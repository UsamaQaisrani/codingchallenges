use clap::Parser;
use load_balancer::backend::start;

#[derive(Parser)]
struct Args {
    #[arg(short = '1')]
    port1: u32,

    #[arg(short = '2')]
    port2: u32,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let be1 = tokio::spawn(start(1, args.port1));
    let be2 = tokio::spawn(start(2, args.port2));

    let (be1, be2) = tokio::join!(be1, be2);

    println!("Backend 1: {:?}", be1);
    println!("Backend 2: {:?}", be2);
}
