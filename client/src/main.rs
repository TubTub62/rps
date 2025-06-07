use tokio;
use tokio::signal;
mod client;

#[tokio::main]
async fn main() {

    let n = 10;
    println!("Spawning {} Clients", &n);

    for i in 0..n {
        println!("Spawned: {}", &i);
        tokio::spawn(async move {
            client::spawn_client(&i).await;
        });
    }

    /* loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(60));
    } */

    signal::ctrl_c().await;
    println!("Shutting Down");

}