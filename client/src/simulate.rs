use super::rps_client::client::*;

use tokio::signal;

pub async fn simulate_clients(n : i32) {
    for i in 0..n {

        //let ip = "127.0.0.1";
        //let free_addr = get_free_ip_non_stream(ip);

        let sim_name = format!("sim_client_{}", i);
        tokio::spawn(async move {
            client(sim_name, true).await;
        });
    }

    signal::ctrl_c().await.expect("???");
    println!("\nShutting Server Down");
}
