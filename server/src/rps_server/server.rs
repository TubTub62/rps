use serde::Serialize;
use tokio::net::{TcpListener, TcpStream};
use tokio::io::AsyncWriteExt;
use tokio::sync::mpsc::{Sender, Receiver};
use tokio::sync::mpsc;

use std::net::{SocketAddr};

use super::rps_match::{RpsMatchClientInfo, RpsMatchClientPair, RpsMatchInfo};
use super::rps_match::{RpsClientStatus, RpsMatchStatus};

use super::rps_match::RpsMoveType;
use super::rps_match::RpsMoveType::*;

use super::rps_match::RpsMoveResult;
use super::rps_match::RpsMoveResult::*;

pub async fn recieve_buf_stream(stream : &TcpStream) -> Vec<u8>{
    let mut comm_buf = Vec::new(); 
    loop {
        stream.readable().await.expect("Should Be Readable");
        match stream.try_read(&mut comm_buf) {
            Ok(n) if n > 0 => return comm_buf,
            _ => continue,
        }
    }
}

pub async fn send_buf_stream(stream : &mut TcpStream, buf : &[u8]) {
    if let Err(e) = stream.write_all(&buf).await {
			println!("Error Getting Name From Client: {}", e);
			return;
    }
}

pub async fn handle_client(mut stream : TcpStream, addr : SocketAddr, c_to_cm_sender : Sender<RpsMatchClientInfo>) {
    stream.set_nodelay(true).unwrap();

    let wait_msg = "Provide Name".to_string();
    
    println!("Requesting Client Name");
    for _ in 0..10 {
        send_buf_stream(&mut stream, wait_msg.as_bytes()).await;
        println!("r");
    }

    println!("Waiting On Client Name");
    let inc_name = recieve_buf_stream(&stream).await;
    println!("Recieved Client Name");

    let inc_name_processed = String::from_utf8(inc_name).unwrap();

    let (match_info_sender, mut match_info_reciever) = mpsc::channel::<RpsMatchInfo>(100);

    let match_stream = TcpStream::connect(&addr).await.unwrap();

    let ci = RpsMatchClientInfo {   
        stream:match_stream,
        client_name:inc_name_processed,
        client_status:RpsClientStatus::Queueing,
        client_sender:match_info_sender.clone(),
    };

    println!("Sending Client To Queue");
    if let Err(_) = c_to_cm_sender.send(ci).await {
        println!("Reciever Dropped");
        return;
    }

    loop {
        match match_info_reciever.recv().await  {
            Some(msg) => {
                println!("handler recieved: {:?}\nSending To Client", msg.to_string());
                let rps_serialized = serde_json::to_string(&msg).unwrap();
                if let Err(e) = stream.write_all(rps_serialized.as_bytes()).await {
                    println!("Error Sending Match Results: {}", e);
                    return;
                }
            }
            None => println!("Channel Has Been Closed"),
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
			Ok((stream, addr)) => {

                println!("Handling Incoming Client");
                let ci_sender_clone = ci_sender.clone();
                tokio::spawn(async move {
					handle_client(stream, addr, ci_sender_clone).await;
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

        println!("Current rc length: {:?}", ready_clients.len());
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
    
    println!("Asking Clients For Moves");
    
    if let Err(e) = p1.stream.write_all(p1_start.as_bytes()).await {
        println!("During Start {} Stream Has Given error {}", &p1.client_name, e);
        return;
    }
    
    if let Err(e) = p2.stream.write_all(p2_start.as_bytes()).await {
        println!("During Start {} Stream Has Given error {}", &p2.client_name, e);
        return;
    }
    
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
        if let Err(e) = p1.stream.write_all(play_move.clone().as_bytes()).await {
        println!("During Start {} Stream Has Given error {}", &p1.client_name, e);
        return;
        }

        if let Err(e) = p2.stream.write_all(play_move.clone().as_bytes()).await {
        println!("During Start {} Stream Has Given error {}", &p2.client_name, e);
        return;
        }

        // Recieving Player Moves
        let mut p1_move_buf = Vec::new(); 
        p1.stream.readable().await.expect("Should Be Readable");

        let p1_move : RpsMoveType;
        match p1.stream.try_read(&mut p1_move_buf) {
            Ok(n) => {
                p1_move = serde_json::from_str(&String::from_utf8(p1_move_buf).unwrap()).unwrap();
            }
            Err(e) => {
                println!("Error Recieving Player Move: {}", e);
                return;
            }
        }

        let mut p2_move_buf = Vec::new(); 
        p2.stream.readable().await.expect("Should Be Readable");

        let p2_move : RpsMoveType;
        match p1.stream.try_read(&mut p2_move_buf) {
            Ok(n) => {
                p2_move = serde_json::from_str(&String::from_utf8(p2_move_buf).unwrap()).unwrap();
            }
            Err(e) => {
                println!("Error Recieving Player Move: {}", e);
                return;
            }
        }

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
        let rps_match_info_serialized = serde_json::to_string(&m).unwrap();

        if let Err(e) = p1.stream.write_all(rps_match_info_serialized.clone().as_bytes()).await {
            println!("During Start {} Stream Has Given error {}", &p1.client_name, e);
            return;
        }

        if let Err(e) = p2.stream.write_all(rps_match_info_serialized.clone().as_bytes()).await {
            println!("During Start {} Stream Has Given error {}", &p2.client_name, e);
            return;
        }

    }

    // Send Match Results To Clients
    let rps_match_info_serialized = serde_json::to_string(&m).unwrap();

    if let Err(e) = p1.stream.write_all(rps_match_info_serialized.clone().as_bytes()).await {
        println!("During Start {} Stream Has Given error {}", &p1.client_name, e);
        return;
    }

    if let Err(e) = p2.stream.write_all(rps_match_info_serialized.clone().as_bytes()).await {
        println!("During Start {} Stream Has Given error {}", &p2.client_name, e);
        return;
    }
    
    // Close Connections
    p1.stream.shutdown().await;
    p2.stream.shutdown().await;
    
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