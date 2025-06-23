use tokio::net::TcpStream;

use rps_lib::types::*;
use rps_lib::util::{recieve_buf_stream, send_buf_stream};

use super::simulate::random_action;

use super::util::*;

pub async fn wait_for_match(stream : &mut TcpStream) -> Result<(), std::io::Error>{

    let opponent = recieve_buf_stream(stream).await?;
    let oponnent_processed = String::from_utf8(opponent).unwrap();

    println!("Match Found, Your Oponnent Is {}", oponnent_processed);

    send_buf_stream(stream, "Recieved".as_bytes()).await?;
    Ok(())
}

pub async fn play_match(stream : &mut TcpStream, player_name : String, simulated : bool) -> Result<(), std::io::Error> {

    let mut match_info_buf : Vec<u8>;
    let mut match_info : RpsMatchInfo;
    loop{
        // Signal To Play Move
        let play_signal = String::from_utf8(recieve_buf_stream(stream).await?).expect("Could Not Convert To Utf8");
        assert_eq!(play_signal, "Play Your Move".to_string());

        // Send Move
        let player_move : RpsMoveType;
        if simulated {
            player_move = random_action();
        } else {
            player_move = get_user_move()?;
        }
        let ser_player_move = serialize_move(player_move).await;
        send_buf_stream(stream, ser_player_move.as_bytes()).await?;

        // Recieve Match State
        match_info_buf = recieve_buf_stream(stream).await?;
        match_info =  deserialize_match_info(match_info_buf).await;

        // Send Processed Signal
        send_buf_stream(stream, "Processed".as_bytes()).await?;

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
    Ok(())
}