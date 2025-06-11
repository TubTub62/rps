use tokio;
use tokio::signal;
mod rps_client;
use rps_client::client;

#[tokio::main]
async fn main() {

    /* let n = 10;
    println!("Spawning {} Clients", &n);

    for i in 0..n {
        println!("Spawned: {}", &i);
        tokio::spawn(async move {
            client::spawn_client(i.to_string()).await;
        });
    } */

    /* loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(60));
    } */

    let p_name = rps_client::client::get_player_name().await;
    rps_client::client::spawn_client(p_name).await;

    signal::ctrl_c().await;
    println!("Shutting Down");

}