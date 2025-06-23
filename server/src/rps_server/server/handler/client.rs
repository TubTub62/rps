use tokio::net::{TcpStream};
use tokio::sync::mpsc::{Sender};
use tokio::sync::mpsc;

use std::io;

use rps_lib::util::{recieve_buf_stream, send_buf_stream};
use rps_lib::types::*;

async fn client_choose_action(stream : &mut TcpStream) -> Result<ClientAction, io::Error> {
    let req = "Choose Action";
    send_buf_stream(stream, req.as_bytes()).await?;

    let action = recieve_buf_stream(stream).await?;
    //println!("action: {}", String::from_utf8(action.clone()).expect("Could not convert to string"));
    let action : ClientAction = serde_json::from_slice(&action.as_slice()).expect("Failed To Deserialize");

    Ok(action)
}


pub async fn handle_client(mut stream : TcpStream, c_to_cm_sender : Sender<RpsMatchClientInfo>) -> Result<(), io::Error> {
    stream.set_nodelay(true).unwrap();

    let wait_msg = "Provide Name".to_string();
    send_buf_stream(&mut stream, wait_msg.as_bytes()).await?;

    let inc_name = recieve_buf_stream(&stream).await?;
    let inc_name_processed = String::from_utf8(inc_name).unwrap();

    loop {
        let action = client_choose_action(&mut stream).await?;
        match action {
            ClientAction::Quit => {
                println!("Client Quit");
                return Ok(());
            }
            ClientAction::FindMatch => {
                let (match_info_sender, mut match_info_reciever) = mpsc::channel::<RpsMatchStatus>(100);

                let provide_match_socket = "Provide Match Socket";
                send_buf_stream(&mut stream, provide_match_socket.as_bytes()).await?;
                let match_socket_ip_buf = recieve_buf_stream(&stream).await?;

                let match_socket_ip = String::from_utf8(match_socket_ip_buf).expect("Failed To Convert Buf To Utf8");
                println!("Connecting To Match Socket");
                let match_stream = TcpStream::connect(match_socket_ip).await.expect("Socket Connection Failed");

                let ci = RpsMatchClientInfo {   
                    stream:match_stream,
                    client_name:inc_name_processed.clone(),
                    client_status:RpsClientStatus::Queueing,
                    client_sender:match_info_sender.clone(),
                };

                println!("Sending Client To Queue");
                if let Err(_) = c_to_cm_sender.send(ci).await {
                    println!("Reciever Dropped");
                    return Ok(());
                }

                match match_info_reciever.recv().await  {
                    Some(msg) if msg.eq(&RpsMatchStatus::Done) => {
                        continue;
                    }
                    Some(msg) if msg.eq(&RpsMatchStatus::Ongoing) => {
                        println!("Wrong Status Acquired. Closing Client Connection");
                        return Ok(());
                    }
                    Some(msg) if msg.eq(&RpsMatchStatus::Abrupt) => {
                        println!("Match Was Abruptly Ended");
                    }
                    Some(_) => {
                        println!("Really Bad Signal");
                        return Ok(());
                    }
                    None => {
                        println!("Channel Has Been Closed");
                        return Ok(());
                    }
                }
            }
        }
    }

}