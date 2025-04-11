use dotenv::dotenv;
use tokio::net::TcpListener;

use backend::app;

#[tokio::main]
async fn main() {
    dotenv().ok();

    // running at 0.0.0.0 port 8081
    let addr = TcpListener::bind("0.0.0.0:8081").await.unwrap();
    println!("->> LISTENING on {:#?}\n", addr);

    axum::serve(addr, app()).await.unwrap();
}
