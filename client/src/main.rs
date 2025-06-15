use tokio;

extern crate rps_lib;

mod rps_client;

#[tokio::main]
async fn main() {

    let p_name = rps_lib::util::get_user_input("Enter Player Name:".to_string()).await;
    let _ = rps_client::client::spawn_client(p_name).await;

}