use tokio;

extern crate rps_lib;

mod rps_client;

mod simulate;


#[tokio::main]
async fn main() {

    //let n = 10_i32.pow(4);
    let n = 10000;
    simulate::simulate_clients(n).await;

}