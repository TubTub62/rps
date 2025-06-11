use tokio::net::TcpStream;
use tokio::io;

pub struct RpsMatchStatus {
    player : String,
    opponent : String,
    p_points : i32,
    o_points : i32
}

pub async fn display_score(rps_m : &RpsMatchStatus) {

    println!("Your Score: {:?}", rps_m.p_points);
    println!("Opponent's Score: {}", &rps_m.o_points);

}

// protocol:
// Vec of player names followed by ;
// vec of associated scores, respectfuly, followed by ;
pub async fn recieve_round_result(rps_m : &mut RpsMatchStatus, stream : &TcpStream) {
    let mut buf = Vec::new();
    stream.readable().await;
    stream.try_read_buf(&mut buf);

    println!("{:?}", &buf);
}

pub async fn wait_for_match(stream : &TcpStream) {
    let mut buf = Vec::new();

    loop {
        // Try to read data, this may still fail with `WouldBlock`
        // if the readiness event is a false positive.
        stream.readable().await.expect("Failed To Read");
        match stream.try_read_buf(&mut buf) {
            Ok(0) => break,
            Ok(n) => {
                println!("incoming message: {:?}", String::from_utf8(buf.clone()));
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                continue;
            }
            Err(e) => {
                println!("Some error: {}", e);
                //return e;
            }
        }
    }

    let msg = String::from_utf8(buf).unwrap();
    println!("{:?}", &msg);
}