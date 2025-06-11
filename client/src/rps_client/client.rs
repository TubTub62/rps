use tokio::net::TcpStream;
use std::io;

use crate::rps_client::rps_match;

pub async fn get_player_name() -> String{
    println!("Type your name:");
    let mut input_string = String::new();
    io::stdin().read_line(&mut input_string);
    return input_string;
} 

pub async fn spawn_client(player_name : String) {

    println!("Client {} - Trying to connect to server", &player_name);

    let addr = "127.0.0.1:4000";
    let stream = TcpStream::connect(&addr).await.unwrap();

    println!("Client {} - Connected to server", &player_name);

    rps_match::wait_for_match(&stream).await;
    println!("Found Match!");
}


