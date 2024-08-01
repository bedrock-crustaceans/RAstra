use bedrock_rs::core::int::VAR;
use bedrock_rs::nbt::{self, NbtTag};
use bedrock_rs::proto::connection::Connection;
use bedrock_rs::proto::gamepacket::GamePacket;
use bedrock_rs::proto::login::login_to_server;
use bedrock_rs::proto::login::provider::DefaultLoginProvider;
use bedrock_rs::proto::packets::chunk_radius_updated::ChunkRadiusUpdatedPacket;
use bedrock_rs::proto::packets::level_chunk::LevelChunkPacket;
use bedrock_rs::proto::types::chunk_pos::ChunkPos;
use tokio::main;
use std::collections::HashMap;
use std::net::{SocketAddr, SocketAddrV4};
use std::str::FromStr;
use std::time::Duration;
use bedrock_rs::world::palet::PalettedStorage;

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
        let game_packet = match shard.recv().await {
            Ok(v) => v,
            Err(e) => {
                println!("ConnectionError {:?}", e);
                break;
            }
        };

        match game_packet {
            GamePacket::RequestChunkRadius(packet) => {
                let response = ChunkRadiusUpdatedPacket {
                    chunk_radius: VAR::<u32>::new(packet.chunk_radius_max.into())
                };

                shard.send(GamePacket::ChunkRadiusUpdate(response)).await.unwrap();
            },
            GamePacket::MovePlayer(packet) => {
                println!("{:?} {:?} {:?}", packet.position, packet.rotation, packet.head_rotation);
            },
            GamePacket::SetLocalPlayerAsInitialized(packet) => {
                let mut palette: Vec<NbtTag> = Vec::new();

                let mut fields: HashMap<String, NbtTag> = HashMap::new();
                fields.insert(String::from("name"), NbtTag::String(String::from("minecraft:air")));

                let compound = NbtTag::Compound(fields);
                palette.push(compound);

                let storage = PalettedStorage {
                    blocks: [0; 4096],
                    palette
                };

                // Serialize to chunk data
                let mut chunk_serialized_data: Vec<u8> = Vec::new();
                chunk_serialized_data.push(8); // format version 8
                chunk_serialized_data.push(1); // only 1 storage (further ones are used for water)
                chunk_serialized_data.extend_from_slice(&storage.encode(true));

                let chunk_packet = LevelChunkPacket {
                    chunk_position: ChunkPos::new(0, 0),
                    dimension_id: VAR::<i32>::new(0),
                    sub_chunk_count: VAR::<u32>::new(1),
                    cache_enabled: false,
                    serialized_chunk_data: chunk_serialized_data,
                    client_needs_to_request_subchunks: false,
                    client_request_subchunk_limit: VAR::new(0xFFFFFFFFu32 as i32)
                };

                shard.send(GamePacket::LevelChunk(chunk_packet)).await.unwrap();
            }
            _ => {
                println!("unhandled {:?}", game_packet);
            }
        }
    }
}