use clap::Parser;
use load_balancer::lb;

#[derive(Parser)]
struct Args {
    #[arg(short = 'p')]
    port: u32,

    #[arg(short = 'b')]
    ports: Vec<u32>,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let mut backends: Vec<String> = Vec::new();

    for port in args.ports {
        backends.push(format!("0.0.0.0:{}", port));
    }

    let lb = tokio::spawn(lb::start(backends, args.port));
    let _lb = tokio::join!(lb);

    println!("LoadBalancer: 0.0.0.0:{}", args.port);
}
