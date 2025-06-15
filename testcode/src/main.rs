use tokio::signal;

mod server;
use server::server_test;

mod client;
use client::client;


async fn get_mode() -> String {
    let mut mode = String::new();
    let _ = std::io::stdin().read_line(&mut mode);
    return mode.replace("\n", "");
}

#[tokio::main]
async fn main(){
    let mode = get_mode().await;
    let str_mode = mode.as_str();
    //assert_eq!(str_mode, "server");

    match str_mode {
        "server" => server_test().await,
        "client" => client().await, 
        _ => println!("Invalid Mode"),
    }
    signal::ctrl_c().await.expect("Could Not Listen");

}