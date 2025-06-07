use chrono::Local;
use tokio::time::{sleep, Duration};
use tokio::{
  io::AsyncWriteExt,
  net::{TcpListener, TcpStream},
};


async fn process(mut socket: TcpStream) {
  println!("Connected this client, Remote Addr: {:?}", socket.peer_addr().unwrap());
  socket.set_nodelay(true).unwrap();

  loop {
    sleep(Duration::from_secs(1)).await;

    let local = Local::now();
    let time = local.format("%H:%M:%S\n").to_string();

    if let Err(e) = socket.write_all(time.as_bytes()).await {
      println!("During send, socket has given error: {}", e);
      return;
    }
  }
}


#[tokio::main]
async fn main() {

    let port = String::from("4000");
    let listener = TcpListener::bind(format!("0.0.0.0:{}", port)).await.unwrap();

    println!("Application started");
    
    loop {
      println!("Listening");
        match listener.accept().await {
        Ok((socket, _)) => {
            tokio::spawn(async move {
            process(socket).await;
            });
        }
        Err(e) => println!("Error in acception: {}", e),
        }
    }

}