use bedrock_rs::proto::connection::Connection;
use bedrock_rs::proto::gamepacket::GamePacket;
use bedrock_rs::proto::listener;
use bedrock_rs::proto::login::login_to_server;
use bedrock_rs::proto::login::provider::DefaultLoginProvider;
use tokio::main;
use std::net::{SocketAddr, SocketAddrV4};
use std::str::FromStr;
use std::time::Duration;

const TICK_INTERVAL: Duration = Duration::from_millis(50);

#[main]
async fn main() {
    let ip = "127.0.0.1:19132";

    let mut listener = bedrock_rs::proto::listener::Listener::new_raknet(
        String::from("My Server"), 
        String::from("RAstra"), 
        100, 
        0, 
        SocketAddr::V4(SocketAddrV4::from_str(ip).unwrap()), 
        false
    ).await.unwrap();

    listener.start().await.unwrap();
    println!("Server running on {}", ip);

    // main loop
    loop {
        let conn = listener.accept().await.unwrap();

        tokio::spawn(async move {
            handle_connection(conn).await;
        });
    }
}

async fn handle_connection(conn: Connection) {
    let mut shard = conn.into_shard(TICK_INTERVAL, 256).await;

    match login_to_server(&mut shard, DefaultLoginProvider::new()).await {
        Ok(_) => {},
        Err(e) => {
            format!("[LoginError] {e:#?}");
            return;
        }
    }

    loop {
        let game_packet = shard.recv().await.unwrap();

        match game_packet {
            GamePacket::PlayerAuthInput(packet) => {
                println!("{:?}", packet.position);
            }
            _ => {
                println!("unhandled {:?}", game_packet);
            }
        }
    }
}