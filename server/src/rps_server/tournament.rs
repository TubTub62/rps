use tokio::net::{TcpStream};
use tokio::sync::mpsc::{Sender, Receiver};

use rps_lib::util::*;
use rps_lib::types::torunament::*;

async fn create_tournament(stream : &mut TcpStream) -> Tournament {
    let req = "Provide Tournament";
    send_buf_stream(stream, req.as_bytes()).await;

    let tourn_raw = recieve_buf_stream(stream).await;
    let tourn : Tournament = serde_json::from_slice(&tourn_raw).expect("Failed To Deserialize");

    return tourn
}

async fn tournament_manager(mut client_rec : Receiver<TournamentClient>) {
    let mut active_tournaments : Vec<Tournament> = vec![];
    loop {
        let tc = client_rec.recv().await.expect("Failed To Recieve");
        let intention = tc.intention;
        let mut stream = tc.stream;
        match intention {
            Intention::Create => {
                let nt = create_tournament(&mut stream).await;
                active_tournaments.push(nt);
                send_buf_stream(&mut stream, "Tournament Created".as_bytes()).await;
            }
            Intention::Fetch => {
                let mut at_ser = Vec::new();
                for t in active_tournaments.clone().into_iter() {
                    let t_ser = serde_json::to_vec(&t).expect("Failed To Serialize");
                    at_ser.push(t_ser);
                }
                send_buf_stream(&mut stream, at_ser.as_slice()).await;
            }
        }
    }
}

async fn tournament_handler() {

}