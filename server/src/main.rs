use chrono::Local;
use tokio::time::{sleep, Duration};
use tokio::net::{TcpListener, TcpStream};
use tokio::io::AsyncWriteExt;

async fn process(mut socket: TcpStream) {
	println!("Connected to client - Remote Addr: {:?}", socket.peer_addr().unwrap());
	socket.set_nodelay(true).unwrap();

	loop {
		sleep(Duration::from_secs(1)).await;

		let local = Local::now();
		let time = local.format("%H:%M:%S\n").to_string();

		if let Err(e) = socket.write_all(time.as_bytes()).await {
			println!("During send, socket has given error: {}", e);
			return;
		}
	}
}


#[tokio::main]
async fn main() {

	let ip = String::from("127.0.0.1");
	let port = String::from("4000");
	let listener = TcpListener::bind(format!("{}:{}", ip, port)).await.unwrap();

	println!("Server Started");

	loop {
		match listener.accept().await {
			Ok((stream, _)) => {
				tokio::spawn(async move {
				process(stream).await;
				});
			}
			Err(e) => println!("Error in acception: {}", e),
		}
	}

}