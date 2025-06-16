use tokio::net::{TcpStream};
use tokio::io::{AsyncWriteExt};
use tokio::time::{sleep, Duration};
use tokio::sync::mpsc::{Sender};

pub async fn get_user_input(msg : String) -> String{
    println!("{}", msg);
    let mut input_string = String::new();
    if let Err(e) = std::io::stdin().read_line(&mut input_string) {
        println!("Some Err: {}", e);
    }
    return input_string.replace("\n", "");
} 

pub async fn recieve_buf_stream(stream : &TcpStream) -> Vec<u8>{
    let mut comm_buf = [0;4096]; 
    loop {
        stream.readable().await.expect("Should Be Readable");
        match stream.try_read(&mut comm_buf) {
            Ok(n) if n > 0 => return comm_buf[..n].to_vec(),
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                //println!("Would Block Err: {}", e);
                sleep(Duration::from_secs(1)).await;
                continue;
            }
            Err(e) => println!("Error Trying To Read: {}", e),
            Ok(_) => sleep(Duration::from_secs(1)).await,
        }
    }
}

pub async fn send_buf_stream(stream : &mut TcpStream, buf : &[u8]) {
    if let Err(e) = stream.write_all(&buf).await {
			println!("Error Getting Name From Client: {}", e);
			return;
    }
}

pub async fn channel_send<T>(chn : &Sender<T>, msg : T ) {
    if let Err(e) = chn.send(msg).await {
        println!("Error Sending On Channel: {}", e);
        return;
    }
}