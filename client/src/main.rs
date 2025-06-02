#[allow(unused_imports)]
use std::io::prelude::*;
use std::net::{SocketAddr};
use std::time::Duration;
use std::thread::sleep;

use tokio::{
    net::{TcpStream},
    io::{BufReader, AsyncWriteExt, AsyncBufReadExt}
};



async fn client(socket_address : SocketAddr, clinet_id: usize) {

    let stream = TcpStream::connect(socket_address).await.unwrap();
    let mut stream_buf_reader = BufReader::new(stream);

    loop {
        sleep(Duration::from_secs(1));
        let mut buffer = String::new();
        stream_buf_reader.read_line(&mut buffer).await.unwrap();
        println!("Client {} Recieved Incoming Message:", clinet_id);
        println!("{:?}", buffer);
    }

}

#[tokio::main]
async fn main() {


    let addr = SocketAddr::from(([127, 0, 0, 1], 4000));

    let client_number = 2;

    let mut clients = Vec::with_capacity(client_number);
    println!("Starting Clients");
    for i in 0..client_number {
        
        clients.push(tokio::spawn(async move{
            client(addr, i).await
    }));

    loop {

    }

    }

}
