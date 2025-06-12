use tokio::net::TcpStream;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Deserialize, PartialEq)]
pub enum RpsMatchStatus {
    Ongoing,
    Done,
}

#[derive(Deserialize)]
pub struct RpsMatchInfo {
    pub p1_name : String,
    pub p2_name : String,
    pub p1_score : i32,
    pub p2_score : i32,
    pub status : RpsMatchStatus,
    pub won_round : String,
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