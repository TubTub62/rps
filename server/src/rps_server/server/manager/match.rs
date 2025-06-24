use tokio::sync::mpsc::Receiver;

use rps_lib::{types::RpsMatchClientInfo};

use crate::rps_server::server::handler::r#match::handle_match;

use rps_lib::sql::util::create_pool;

pub async fn match_manager(mut client_stack : Receiver<RpsMatchClientInfo>) {

    create_pool(5).await;

    let mut ready_clients = Vec::new();
    loop {
        match client_stack.recv().await {
            Some(c) => {
                println!("Match Manager: Recieved Client To Queue");
                ready_clients.push(c);
            }
            None => println!("Match Manager: Recieved Nothing"),
        }

        //println!("Current Queue Length: {:?}", ready_clients.len());
        while ready_clients.len() >= 2 {
            println!("Setting Up Match");
            let mut proc = ready_clients.drain(0..2);
            let p1 = proc.next().unwrap();
            let p2 = proc.next().unwrap();
            tokio::spawn(async move {
                handle_match(p1, p2).await
            });
        }
    }
}