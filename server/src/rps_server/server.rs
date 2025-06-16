use tokio::net::{TcpListener, TcpStream};
use tokio::io::AsyncWriteExt;
use tokio::sync::mpsc::{Sender, Receiver};
use tokio::sync::mpsc;

extern crate rps_lib;

use rps_lib::types::{ClientAction, RpsMatchClientInfo, RpsMatchClientPair, RpsMatchInfo};
use rps_lib::types::{RpsClientStatus, RpsMatchStatus};

use rps_lib::types::RpsMoveType;
use rps_lib::types::RpsMoveType::*;

use rps_lib::types::RpsMoveResult;
use rps_lib::types::RpsMoveResult::*;

use rps_lib::util::{channel_send, recieve_buf_stream, send_buf_stream};

pub async fn client_choose_action(stream : &mut TcpStream) -> ClientAction {
    let req = "Choose Action";
    send_buf_stream(stream, req.as_bytes()).await;

    let action = recieve_buf_stream(stream).await;
    let action : ClientAction = serde_json::from_slice(&action.as_slice()).expect("Failed To Deserialize");

    return action;
}

pub async fn handle_client(mut stream : TcpStream, c_to_cm_sender : Sender<RpsMatchClientInfo>) {
    stream.set_nodelay(true).unwrap();

    let wait_msg = "Provide Name".to_string();
    send_buf_stream(&mut stream, wait_msg.as_bytes()).await;

    let inc_name = recieve_buf_stream(&stream).await;
    let inc_name_processed = String::from_utf8(inc_name).unwrap();

    loop {
        let action = client_choose_action(&mut stream).await;
        match action {
            ClientAction::Quit => {
                println!("Client Quit");
                return;
            }
            ClientAction::FindMatch => {
                let (match_info_sender, mut match_info_reciever) = mpsc::channel::<RpsMatchStatus>(100);

                let match_socket_ip_buf = recieve_buf_stream(&stream).await;
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
                    return;
                }

                match match_info_reciever.recv().await  {
                    Some(msg) if msg.eq(&RpsMatchStatus::Done) => {
                        continue;
                    }
                    Some(msg) if msg.eq(&RpsMatchStatus::Ongoing) => {
                        println!("Wrong Status Acquired. Closing Client Connection");
                        return;
                    }
                    Some(_) => {
                        println!("Really Bad Signal");
                        return;
                    }
                    None => {
                        println!("Channel Has Been Closed");
                        return;
                    }
                }
            }
        }

    
    }

}

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
					handle_client(stream, ci_sender_clone).await;
				});
			}
			Err(e) => println!("Error in acception: {}", e),
		}
	}
}

pub async fn match_manager(mut client_stack : Receiver<RpsMatchClientInfo>) {

    let mut ready_clients = Vec::new();
    loop {
        match client_stack.recv().await {
            Some(c) => {
                println!("Match Manager: Recieved Client To Queue");
                ready_clients.push(c);
            }
            None => println!("Match Manager: Recieved Nothing"),
        }

        println!("Current Queue Length: {:?}", ready_clients.len());
        while ready_clients.len() >= 2 {
            println!("Setting Up Match");
            let mut proc = ready_clients.drain(0..2);
            let p1 = proc.next().unwrap();
            let p2 = proc.next().unwrap();
            let rps_client_pair = RpsMatchClientPair { p1:p1, p2:p2 };
            tokio::spawn(async move {
                handle_match(rps_client_pair).await;
            });
        }
    }
}

pub async fn handle_match(client_pair : RpsMatchClientPair) {
    let mut p1 = client_pair.p1;
    let mut p2 = client_pair.p2;
    
    // Giving Each Client Oponnent Name
    let p1_start = format!("{}", p2.client_name.clone());
    let p2_start = format!("{}", p1.client_name.clone());
    
    println!("Sending Opponent Names");
    
    send_buf_stream(&mut p1.stream, p1_start.as_bytes()).await;
    send_buf_stream(&mut p2.stream, p2_start.as_bytes()).await;

    let rec1 = recieve_buf_stream(&p1.stream).await;
    let rec2 = recieve_buf_stream(&p2.stream).await;

    assert_eq!(rec1, "Recieved".as_bytes());
    assert_eq!(rec2, "Recieved".as_bytes());

    let play_move = "Play Your Move".to_string();
    let mut m = RpsMatchInfo { 
        p1_name:p1.client_name.clone(),
        p2_name:p2.client_name.clone(), 
        p1_score:0, 
        p2_score:0, 
        status:RpsMatchStatus::Ongoing,
        won_round:"None".to_string(),
    };
    loop {
        // Requesting Player Moves
        println!("Requsting Player Moves");
        send_buf_stream(&mut p1.stream, play_move.clone().as_bytes()).await;
        send_buf_stream(&mut p2.stream, play_move.clone().as_bytes()).await;

        // Recieving Player Moves
        let p1_move_buf = recieve_buf_stream(&p1.stream).await; 
        let p1_move_string = String::from_utf8(p1_move_buf).expect("Could Not Convert To String");
        let p1_move : RpsMoveType = serde_json::from_str(&p1_move_string).expect("Could Not Convert To RpsMoveType");

        let p2_move_buf = recieve_buf_stream(&p2.stream).await; 
        let p2_move : RpsMoveType = serde_json::from_str(&String::from_utf8(p2_move_buf)
            .expect("Could Not Convert To String"))
            .expect("Could Not Convert To RpsMoveType");

        // Check Who Wins Round
        let (p1_res, p2_res) = who_wins_move(p1_move, p2_move);

        // Update Match Info
        if p1_res == Win {
            m.p1_score += 1;
            m.won_round = m.p1_name.clone();
        }

        if p2_res == Win {
            m.p2_score += 1;
            m.won_round = m.p2_name.clone();
        }

        if m.p1_score >= 2 || m.p2_score >= 2 {
            m.status = RpsMatchStatus::Done;
            break;
        }


        // Send Match Info To Players
        let rps_lib_info_serialized = serde_json::to_string(&m).unwrap();

        send_buf_stream(&mut p1.stream, rps_lib_info_serialized.clone().as_bytes()).await;
        send_buf_stream(&mut p2.stream, rps_lib_info_serialized.clone().as_bytes()).await;

        let proc1 = recieve_buf_stream(&p1.stream).await;
        let proc2 = recieve_buf_stream(&p2.stream).await;

        assert_eq!(proc1, "Processed".as_bytes());
        assert_eq!(proc2, "Processed".as_bytes());
    }

    // Send Match Results To Clients
    let rps_lib_info_serialized = serde_json::to_string(&m).unwrap();

    send_buf_stream(&mut p1.stream, rps_lib_info_serialized.clone().as_bytes()).await;
    send_buf_stream(&mut p2.stream, rps_lib_info_serialized.clone().as_bytes()).await;
    
    // Sending Signal To Client Handler
    channel_send(&p1.client_sender, RpsMatchStatus::Done).await;
    channel_send(&p2.client_sender, RpsMatchStatus::Done).await;

    // Close Connections
    p1.stream.shutdown().await.expect("Shutdown of p1 stream failed");
    p2.stream.shutdown().await.expect("Shutdonw of p2 stream failed");
    
    println!("Match Finished");
}

pub fn who_wins_move(p1_move : RpsMoveType, p2_move : RpsMoveType) -> (RpsMoveResult, RpsMoveResult) {
    match p1_move {
        Rock => {
            match p2_move {
                Rock   => return (Draw, Draw),
                Paper  => return (Lose, Win),
                Scissor => return (Win, Lose),
            }
        }
        Paper => {
            match p2_move {
                Rock    => return (Win, Lose),
                Paper   => return (Draw, Draw),
                Scissor => return (Lose, Win),
            }
        }
        Scissor => {
            match p2_move {
                Rock    => return (Lose, Win),
                Paper   => return (Win, Lose),
                Scissor => return (Draw, Draw),
            }
        }
    }
}