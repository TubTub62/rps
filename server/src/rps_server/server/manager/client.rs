use tokio::net::{TcpListener};
use tokio::sync::mpsc;

use rps_lib::types::RpsMatchClientInfo;

use crate::rps_server::server::handler;
use crate::rps_server::server::manager::r#match::match_manager;

pub async fn client_manger() {

    let ip = String::from("127.0.0.1");
	let port = String::from("4000");
	let listener = TcpListener::bind(format!("{}:{}", ip, port)).await.unwrap();

    //ci_sender is for cm -> mm // ci_reciever is for giving mm clients
    // ClientInfo
    let (ci_sender, ci_reciever) = mpsc::channel::<RpsMatchClientInfo>(100); 

    println!("Spawning Match Manager");
    tokio::spawn(async move {
        match_manager(ci_reciever).await;
    });


	loop {
		match listener.accept().await {
			Ok((stream, _)) => {

                println!("Handling Incoming Client");
                let ci_sender_clone = ci_sender.clone();
                tokio::spawn(async move {
					let _ = handler::client::handle_client(stream, ci_sender_clone).await;
				});
			}
			Err(e) => println!("Error in acception: {}", e),
		}
	}
}