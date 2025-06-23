use tokio::{net::{TcpListener, TcpStream}};

extern crate rps_lib;
use rps_lib::{types::{ClientAction}, util::{recieve_buf_stream, send_buf_stream}};

use super::util::*;
use super::r#match::*;

pub async fn client(player_name : String, simulated : bool) -> Result<(), std::io::Error>{

    let addr = "127.0.0.1:4000";
    let mut stream = TcpStream::connect(addr).await.unwrap();
    println!("{} - Connected to server on remote address: {}", player_name, stream.peer_addr().expect("Could not get remote address"));

    let server_request = recieve_buf_stream(&stream).await?;
    let server_request_processed = String::from_utf8(server_request).unwrap();
    assert_eq!(server_request_processed, "Provide Name".to_string());
    send_buf_stream(&mut stream, player_name.clone().as_bytes()).await;

    loop {
        let act_signal = recieve_buf_stream(&stream).await?;
        assert_eq!(act_signal, "Choose Action".as_bytes());
        let action : ClientAction;
        if simulated {
            action = ClientAction::FindMatch
        } else {
            action = user_choose_action().await?;
        }
        match action {
            ClientAction::Quit => {
                let action_ser = serde_json::to_string(&ClientAction::Quit).expect("Failed To Serialize");
                send_buf_stream(&mut stream, action_ser.as_bytes()).await?;
                println!("Quiting RPS");
                return Ok(())
            }
            ClientAction::FindMatch => {
                let action_ser = serde_json::to_string(&ClientAction::FindMatch).expect("Failed To Serialize");
                send_buf_stream(&mut stream, action_ser.as_bytes()).await?;
                
                let recieve_signal = recieve_buf_stream(&stream).await?;
                assert_eq!(recieve_signal, "Provide Match Socket".as_bytes());

                let free_ip = get_free_ip(&stream).await;
                let match_listener = TcpListener::bind(free_ip.clone()).await.expect("Could Not Create Match Listener");
                send_buf_stream(&mut stream, free_ip.clone().as_bytes()).await;

                let mut match_stream : TcpStream;
                match match_listener.accept().await {
                    Ok((ms, _)) => {
                        match_stream = ms
                    }
                    Err(e) => {
                        println!("Some Err: {}", e);
                        return Err(e);
                    }
                }
                wait_for_match(&mut match_stream).await;
                play_match(&mut match_stream, player_name.clone(), simulated).await;
                drop(match_listener);
            }
        }
    }
}


