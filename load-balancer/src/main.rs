use clap::Parser;
use load_balancer::{backend::start, lb};

#[derive(Parser)]
struct Args {
    #[arg(short = '1')]
    port1: u32,

    #[arg(short = '2')]
    port2: u32,

    #[arg(short = '3')]
    port3: u32,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let be1 = tokio::spawn(start(1, args.port1));
    let be2 = tokio::spawn(start(2, args.port2));

    let backends = vec![
        format!("0.0.0.0:{}", args.port1),
        format!("0.0.0.0:{}", args.port2),
    ];
    let lb = tokio::spawn(lb::start(backends, args.port3));
    let (be1, be2, lb) = tokio::join!(be1, be2, lb);

    println!("Backend 1: {:?}", be1);
    println!("Backend 2: {:?}", be2);
}
