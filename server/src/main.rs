use tokio::signal;

mod rps_server;
use rps_server::server::{client_manger};

#[tokio::main]
async fn main() {

	println!("Spawning Client Manager");
	client_manger().await;

	signal::ctrl_c().await.expect("???");
    println!("Shutting Server Down");

}