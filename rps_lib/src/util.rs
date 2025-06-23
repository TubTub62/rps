use tokio::net::{TcpStream};
use tokio::io::{AsyncWriteExt};
use tokio::time::{sleep, Duration};
use tokio::sync::mpsc::{Sender};

use tokio::io;
use tokio::sync::mpsc::error::SendError;

pub fn get_user_input(msg : String) -> Result<String, io::Error>{
    println!("{}", msg);
    let mut input_string = String::new();
    std::io::stdin().read_line(&mut input_string)?;
    return Ok(input_string.replace("\n", ""));
} 

pub async fn recieve_buf_stream(stream : &TcpStream) -> Result<Vec<u8>, io::Error> {
    let mut comm_buf = [0;4096]; 
    loop {
        stream.readable().await.expect("Should Be Readable");
        match stream.try_read(&mut comm_buf) {
            Ok(n) if n > 0 => return Ok(comm_buf[..n].to_vec()),
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                //println!("Would Block Err: {}", e);
                sleep(Duration::from_secs(1)).await;
                continue;
            }
            Err(e) => {
                return Err(e);
            }
            Ok(_) => sleep(Duration::from_secs(1)).await,
        }
    }
}

pub async fn send_buf_stream(stream : &mut TcpStream, buf : &[u8]) -> io::Result<()> {
    stream.write_all(&buf).await?;
    Ok(())
}

pub async fn channel_send<T>(chn : &Sender<T>, msg : T ) -> Result<(), SendError<T>> {
    chn.send(msg).await?;
    Ok(())
}