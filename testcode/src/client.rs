use std::time::Duration;

use tokio::{net::TcpStream};
use tokio::time::sleep;

const IP : &str = "127.0.0.1:8080";

pub async fn client(){
    let socket = TcpStream::connect(IP).await.expect("Could Not Connect");
    print!("Client Remote Address: {:?}", socket.peer_addr());

    let mut buf = [0; 4096];
    loop {
        socket.readable().await.expect("Not Readable");
        match socket.try_read(&mut buf) {
            Ok(n) => println!("Ok: {}", n),
            Err(e) => println!("Some Err: {}", e),
        }
        println!("Recieved: {}", String::from_utf8(buf.clone().to_vec()).expect("Expected A utf8 String"));
        sleep(Duration::from_secs(1)).await;
    }
}