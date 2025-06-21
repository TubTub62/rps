use tokio::{net::{TcpListener, TcpStream}};

extern crate rps_lib;
use rps_lib::{types::{ClientAction, RpsMoveType}, util::{get_user_input, recieve_buf_stream, send_buf_stream}};
use rps_lib::types::{RpsMatchInfo};
use rps_lib::types::RpsMoveType::*;

use port_check;

use super::simulate::random_action;

pub async fn get_free_ip(stream : &TcpStream) -> String {
    let free_port = port_check::free_local_port().expect("Failed To Get An Open Port");
    
    let client_ip : String =  stream
        .local_addr()
        .unwrap()
        .to_string()
        .clone();

    let ip_temp : Vec<&str> = client_ip 
        .as_str()
        .split(":")
        .collect();

    let ip = ip_temp[0];
    let full_new_addr = format!("{}:{}", ip, free_port);
    return full_new_addr;
}

pub async fn wait_for_match(stream : &mut TcpStream) {

    let opponent = recieve_buf_stream(stream).await;
    let oponnent_processed = String::from_utf8(opponent).unwrap();

    println!("Match Found, Your Oponnent Is {}", oponnent_processed);

    send_buf_stream(stream, "Recieved".as_bytes()).await;

}

pub async fn convert_to_move(player_move : String) -> Result<RpsMoveType, String> {
    match player_move.as_str() {
        "Rock"      => Ok(Rock),
        "Paper"     => Ok(Paper),
        "Scissor"   => Ok(Scissor),
        _           => Err("Invalid Input".to_string()),
    }
}

pub async fn deserialize_match_info(buf : Vec<u8>) -> RpsMatchInfo {
    let s = String::from_utf8(buf).expect("Failed To Convert To String");
    let ds : RpsMatchInfo = serde_json::from_str(&s.as_str()).expect("Failed To Deserialize");
    return ds;
}

pub async fn serialize_move(p_move : RpsMoveType) -> String {
    let s_move = serde_json::to_string(&p_move).expect("Failed To Serialize");
    return s_move;
}

pub async  fn display_results(mi : RpsMatchInfo, player_name : String) {
    match player_name {
        client_name if client_name.eq(&mi.p1_name) => {
            println!("{} Won Round!", mi.won_round);
            println!("Score: {} - {}", mi.p1_score, mi.p2_score);
        }
        client_name if client_name.eq(&mi.p2_name) => {
            println!("{} Won Round!", mi.won_round);
            println!("Score: {} - {}", mi.p2_score, mi.p1_score);
        }
        _ => {
            println!("Wrong Info");
        }
    }
}

pub async fn play_match(stream : &mut TcpStream, player_name : String, simulated : bool) {

    let mut match_info_buf : Vec<u8>;
    let mut match_info : RpsMatchInfo;
    loop{
        // Signal To Play Move
        let play_signal = String::from_utf8(recieve_buf_stream(stream).await).expect("Could Not Convert To Utf8");
        assert_eq!(play_signal, "Play Your Move".to_string());

        // Send Move
        let player_move : RpsMoveType;
        if simulated {
            player_move = random_action();
        } else {
            let player_move_raw = get_user_input("Play Move".to_string()).await;
            player_move = convert_to_move(player_move_raw).await.unwrap();
        }
        let ser_player_move = serialize_move(player_move).await;
        send_buf_stream(stream, ser_player_move.as_bytes()).await;

        // Recieve Match State
        match_info_buf = recieve_buf_stream(stream).await;
        match_info =  deserialize_match_info(match_info_buf).await;

        // Send Processed Signal
        send_buf_stream(stream, "Processed".as_bytes()).await;

        // Check If Match Ended
        if match_info.status == rps_lib::types::RpsMatchStatus::Done {
            break;
        }

        //Display Results
        display_results(match_info, player_name.clone()).await;
    }

    // Display Final Results
    let res = match_info.won_round.clone();
    display_results(match_info, player_name.clone()).await;
    println!("{} Won The Match!", res);

}

pub async fn user_choose_action() -> ClientAction {
    loop {
        println!("1. Quit");
        println!("2. Find Match");
        let user_action_input = get_user_input("Choose Action:".to_string())
            .await
            .replace("\n", "");

        let conv : i32;
        match user_action_input.parse() {
            Ok(n) => conv = n,
            Err(_) => {
                println!("Wrong Input");
                continue;
            }
        }

        match conv {
            1 => {
                return ClientAction::Quit;
            }
            2 => {
                return ClientAction::FindMatch;
            }
            _ => {
                println!("Wrong Input");
                continue;
            }
        }
    }
}

pub async fn client(player_name : String, simulated : bool) {

    let addr = "127.0.0.1:4000";
    let mut stream = TcpStream::connect(addr).await.unwrap();
    println!("{} - Connected to server on remote address: {}", player_name, stream.peer_addr().expect("Could not get remote address"));

    let server_request = recieve_buf_stream(&stream).await;
    let server_request_processed = String::from_utf8(server_request).unwrap();
    assert_eq!(server_request_processed, "Provide Name".to_string());
    send_buf_stream(&mut stream, player_name.clone().as_bytes()).await;

    loop {
        let act_signal = recieve_buf_stream(&stream).await;
        assert_eq!(act_signal, "Choose Action".as_bytes());
        let action : ClientAction;
        if simulated {
            action = ClientAction::FindMatch
        } else {
            action = user_choose_action().await;
        }
        match action {
            ClientAction::Quit => {
                let action_ser = serde_json::to_string(&ClientAction::Quit).expect("Failed To Serialize");
                send_buf_stream(&mut stream, action_ser.as_bytes()).await;
                println!("Quiting RPS");
                return;
            }
            ClientAction::FindMatch => {
                let action_ser = serde_json::to_string(&ClientAction::FindMatch).expect("Failed To Serialize");
                send_buf_stream(&mut stream, action_ser.as_bytes()).await;
                
                let recieve_signal = recieve_buf_stream(&stream).await;
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
                        return;
                    }
                }
                
                wait_for_match(&mut match_stream).await;
                play_match(&mut match_stream, player_name.clone(), simulated).await;
                drop(match_listener);
            }
        }
    }

    
}


