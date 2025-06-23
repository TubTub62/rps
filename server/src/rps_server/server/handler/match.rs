use tokio::io::AsyncWriteExt;

use std::io;

use rps_lib::types::*;
use rps_lib::types::RpsMoveType::*;
use rps_lib::types::RpsMoveResult::*;
use rps_lib::util::{recieve_buf_stream, send_buf_stream};
use rps_lib::util::channel_send;

pub async fn handle_match(mut p1 : RpsMatchClientInfo, mut p2 : RpsMatchClientInfo) -> Result<(), io::Error> {
    
    // Giving Each Client Oponnent Name
    let p1_start = format!("{}", p2.client_name.clone());
    let p2_start = format!("{}", p1.client_name.clone());
    
    println!("Sending Opponent Names");
    
    send_buf_stream(&mut p1.stream, p1_start.as_bytes()).await?;
    send_buf_stream(&mut p2.stream, p2_start.as_bytes()).await?;

    let rec1 = recieve_buf_stream(&p1.stream).await?;
    let rec2 = recieve_buf_stream(&p2.stream).await?;

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
        send_buf_stream(&mut p1.stream, play_move.clone().as_bytes()).await?;
        send_buf_stream(&mut p2.stream, play_move.clone().as_bytes()).await?;

        // Recieving Player Moves
        let p1_move_buf = recieve_buf_stream(&p1.stream).await?; 
        let p1_move_string = String::from_utf8(p1_move_buf).expect("Could Not Convert To String");
        let p1_move : RpsMoveType = serde_json::from_str(&p1_move_string).expect("Could Not Convert To RpsMoveType");

        let p2_move_buf = recieve_buf_stream(&p2.stream).await?; 
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

        send_buf_stream(&mut p1.stream, rps_lib_info_serialized.clone().as_bytes()).await?;
        send_buf_stream(&mut p2.stream, rps_lib_info_serialized.clone().as_bytes()).await?;

        let proc1 = recieve_buf_stream(&p1.stream).await?;
        let proc2 = recieve_buf_stream(&p2.stream).await?;

        assert_eq!(proc1, "Processed".as_bytes());
        assert_eq!(proc2, "Processed".as_bytes());
    }

    // Send Match Results To Clients
    let rps_lib_info_serialized = serde_json::to_string(&m).unwrap();

    send_buf_stream(&mut p1.stream, rps_lib_info_serialized.clone().as_bytes()).await?;
    send_buf_stream(&mut p2.stream, rps_lib_info_serialized.clone().as_bytes()).await?;
    
    // Sending Signal To Client Handler
    if let Err(_) = channel_send(&p1.client_sender, RpsMatchStatus::Done).await {
        let _ = channel_send(&p2.client_sender, RpsMatchStatus::Abrupt).await;
        return Ok(());
    }
    if let Err(_) = channel_send(&p2.client_sender, RpsMatchStatus::Done).await {
        let _ = channel_send(&p1.client_sender, RpsMatchStatus::Abrupt).await;
        return Ok(());
    }

    // Close Connections
    p1.stream.shutdown().await.expect("Shutdown of p1 stream failed");
    p2.stream.shutdown().await.expect("Shutdonw of p2 stream failed");
    
    // Manually Dropping Streams
    drop(p1);
    drop(p2);

    println!("Match Finished");
    Ok(())
}

fn who_wins_move(p1_move : RpsMoveType, p2_move : RpsMoveType) -> (RpsMoveResult, RpsMoveResult) {
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