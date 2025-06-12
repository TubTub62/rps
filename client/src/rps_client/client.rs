use tokio::{io::AsyncWriteExt, net::TcpStream};
use std::io;

use crate::rps_client::{play, rps_match::{self, RpsMatchInfo}};

pub async fn get_player_name() -> String{
    println!("Type your name:");
    let mut input_string = String::new();
    io::stdin().read_line(&mut input_string);
    return input_string;
} 

pub async fn recieve_buf_from_server(stream : &TcpStream) -> Vec<u8>{
    let mut comm_buf = Vec::new(); 
    stream.readable().await.expect("Should Be Readable");
    stream.try_read(&mut comm_buf);
    return comm_buf;
}

pub async fn send_buf_to_server(stream : &mut TcpStream, buf : &[u8]) {
    if let Err(e) = stream.write_all(&buf).await {
        println!("Error Sending Buf To Server: {}", e);
        return;
    }
}

pub async fn wait_for_match(stream : &TcpStream) -> String {

    let opponent = recieve_buf_from_server(stream).await;
    let oponnent_processed = String::from_utf8(opponent).unwrap();

    println!("Match Found, Your Oponnent Is {}", oponnent_processed);

    return oponnent_processed;

}


pub async fn play_match(stream : &mut TcpStream, player_name : String, oponnent_name : String) {

    loop{
        // Signal To Play Move
        let play_signal = String::from_utf8(recieve_buf_from_server(stream).await).unwrap();
        if play_signal.eq("Play Your Move")  {
            println!("Wrong Signal");
            return;
        };

        // Send Move
        let player_move = play::get_move().await;
        send_buf_to_server(stream, player_move.as_bytes()).await;

        // Recieve Match State
        let match_info_buf = recieve_buf_from_server(stream).await;
        let match_info : RpsMatchInfo=  serde_json::from_str(&String::from_utf8(match_info_buf).unwrap()).unwrap();

        
        
    }



}

pub async fn spawn_client(player_name : String) {

    println!("Client {} - Trying to connect to server", &player_name);

    let addr = "127.0.0.1:4000";
    let mut stream = TcpStream::connect(&addr).await.unwrap();

    println!("Client {} - Connected to server", &player_name);

    let server_request = recieve_buf_from_server(&stream).await;
    let server_request_processed = String::from_utf8(server_request).unwrap();
    if server_request_processed.eq("Provide Name") {
        send_buf_to_server(&mut stream, player_name.clone().as_bytes()).await;
    }
    


    wait_for_match(&stream).await;
    println!("Found Match!");
}


