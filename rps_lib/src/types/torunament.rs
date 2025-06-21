
use tokio::net::{TcpStream};

use serde::{Deserialize, Serialize};

use crate::types::RpsMatchInfo;

pub enum Intention {
    Create,
    Fetch,
}

pub struct TournamentClient {
    pub stream : TcpStream,
    pub intention : Intention,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Tournament {
    name : String,
    player_count : i32,
    tournament_winner : String,
    bracket : Vec<Vec<RpsMatchInfo>>, // descending order
}

impl Tournament {
    pub fn new(n : String, pc : i32, tw : String, brac : Vec<Vec<RpsMatchInfo>>) -> Tournament {
        let t = Tournament{
            name:n,
            player_count:pc,
            tournament_winner:tw,
            bracket:brac,
        };
        return t;
    }
    
}
