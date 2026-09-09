use load_balancer::backend::start;

#[tokio::main]
async fn main() {
    let be1 = tokio::spawn(start(1));
    let be2 = tokio::spawn(start(2));

    let (be1, be2) = tokio::join!(be1, be2);

    println!("Backend 1: {:?}", be1);
    println!("Backend 2: {:?}", be2);
}
