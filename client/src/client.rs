use tokio::net::TcpStream;

pub async fn spawn_client(id : &i32) {

println!("Cleint {} - Trying to connect to server", &id);

let addr = "127.0.0.1:4000";
let stream = TcpStream::connect(&addr).await.unwrap();

println!("Client {} - Connected to server", &id);

loop {
    let mut buf = Vec::new();
    stream.readable().await;
    stream.try_read_buf(&mut buf);

    println!("Client {} - Recieved Message: {:?}", &id, &buf);
    }
}


