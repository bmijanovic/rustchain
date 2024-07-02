use std::sync::Arc;
use tokio::sync::{Mutex};
use libp2p::gossipsub::{Event, IdentTopic};
use warp::http::StatusCode;
use crate::Node;
use crate::types::dto::BlockchainData;

pub async fn hello_world() -> Result<impl warp::Reply, warp::Rejection> {
    Ok(warp::reply::json(&"Hello, world!"))
}

// print the blockchain from node
pub async fn print_blockchain(node: Arc<Mutex<Node>>) -> Result<impl warp::Reply, warp::Rejection> {
    let node = node.lock().await;
    Ok(warp::reply::with_status(warp::reply::json(&node.blockchain), StatusCode::OK))

}

pub async fn mine_block(node: Arc<Mutex<Node>>, data: BlockchainData) -> Result<impl warp::Reply, warp::Rejection> {
    let mut node = node.lock().await;
    let new_block = node.blockchain.add_block(data.data);
    let mut new_block_json = serde_json::to_string(&new_block).unwrap();
    new_block_json = "blockchain: ".to_string() + &new_block_json;
    node.event_sender.as_ref().unwrap().send(new_block_json).await
        .expect("Failed to send message to event sender");
    Ok(warp::reply::with_status(warp::reply::json(&new_block), StatusCode::CREATED))
}