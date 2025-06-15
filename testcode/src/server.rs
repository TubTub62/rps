use std::time::Duration;

use tokio::{net::{TcpListener, TcpStream}};
use tokio::time::sleep;

const IP : &str ="127.0.0.1:8080";
const TEST_STRING : &str = "test_message_from_server";

async fn proc(socket : TcpStream) {
    println!("Server Remote Address: {:?}", socket.peer_addr());
    loop {
        println!("Sending Message");
        println!("{:?}", TEST_STRING);
        socket.writable().await.expect("Not Writeable");
        
        match socket.try_write(TEST_STRING.as_bytes()) {
            Ok(n) => println!("Ok: {}", n),
            Err(e) => println!("Some Err: {}", e),
        }
           
        sleep(Duration::from_secs(1)).await;
    }
}

pub async fn server_test(){
    let listener = TcpListener::bind(IP).await.expect("Could Not Create Listener");

    loop {
        match listener.accept().await {
            Ok((socket, _)) => {
                tokio::spawn(async move {
                    proc(socket).await;
                });
            }
            Err(e) => println!("Some error: {e}"),
        }
    }
}