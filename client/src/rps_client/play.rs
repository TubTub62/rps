use tokio::net::{TcpStream};
use tokio::io::AsyncWriteExt;
use std::io;

pub async fn get_move() -> String{
    println!("Type in your move");
    let mut input_string = String::new();
    io::stdin().read_line(&mut input_string);
    return input_string;
} 

pub async fn play_move(rcp_move : String, stream : TcpStream) {
    //stream.write_all(rcp_move).await;
}