use tokio::{net::{TcpStream}};

use port_check;

use rps_lib::types::*;
use rps_lib::types::RpsMoveType::*;
use rps_lib::util::get_user_input;


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

fn convert_to_move(player_move : String) -> Result<RpsMoveType, String> {
    match player_move.as_str() {
        "Rock"      => Ok(Rock),
        "Paper"     => Ok(Paper),
        "Scissor"   => Ok(Scissor),
        _           => Err("Invalid Input".to_string()),
    }
}

pub fn get_user_move() -> Result<RpsMoveType, std::io::Error> {
    loop {
        let player_move_raw = get_user_input("Play Move".to_string())?;
        match convert_to_move(player_move_raw) {
            Ok(some_move) => {
                return Ok(some_move);
            }
            Err(_) => continue,
        }
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

pub async fn user_choose_action() -> Result<ClientAction, std::io::Error> {
    loop {
        println!("1. Quit");
        println!("2. Find Match");
        let user_action_input = get_user_input("Choose Action:".to_string())?.replace("\n", "");

        let conv : i32;
        match user_action_input.parse() {
            Ok(n) => conv = n,
            Err(_) => {
                println!("Wrong Input");
                continue;
            }
        }

        match conv {
            1 => return Ok(ClientAction::Quit),
            2 => return Ok(ClientAction::FindMatch),
            _ => {
                println!("Wrong Input");
                continue;
            }
        }
    }
}

