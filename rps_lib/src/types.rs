use tokio::net::TcpStream;
use tokio::sync::mpsc::{Sender};
use serde::{Deserialize, Serialize};

pub mod torunament;

pub enum RpsClientStatus {
    Queueing,
    FinishedMatch,
}

#[derive(Copy, Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum RpsMatchStatus {
    Ongoing,
    Done,
    Abrupt,
}

pub struct RpsMatchClientInfo {
    pub stream : TcpStream,
    pub client_name : String,
    pub client_status : RpsClientStatus,
    pub client_sender : Sender<RpsMatchStatus>,
}


pub struct RpsMatchClientPair {
    pub p1 : RpsMatchClientInfo,
    pub p2 : RpsMatchClientInfo,
}

#[derive(Serialize, Deserialize)]
pub struct RpsMatchInfo {
    pub p1_name : String,
    pub p2_name : String,
    pub p1_score : i32,
    pub p2_score : i32,
    pub status : RpsMatchStatus,
    pub won_round : String,
}

impl RpsMatchInfo {
    pub fn to_string(&self)  -> String {
        return format!("Client Name: {}", self.p1_name);
    }
}

impl Clone for RpsMatchInfo {
    fn clone(&self) -> RpsMatchInfo {
        return RpsMatchInfo{ 
            p1_name:self.p1_name.clone(), 
            p2_name:self.p2_name.clone(), 
            p1_score:self.p1_score, 
            p2_score:self.p2_score, 
            status:self.status,
            won_round:self.won_round.clone(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub enum RpsMoveType {
    Rock,
    Paper,
    Scissor,
}

#[derive(PartialEq)]
pub enum RpsMoveResult {
    Win,
    Draw,
    Lose
}


#[derive(Serialize, Deserialize)]
pub enum ClientAction {
    Quit,
    FindMatch,
}

