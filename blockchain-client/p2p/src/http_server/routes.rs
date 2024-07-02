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
    let blockchain = node.blockchain.read().await.clone();
    Ok(warp::reply::with_status(warp::reply::json(&blockchain), StatusCode::OK))
}

pub async fn mine_block(node: Arc<Mutex<Node>>, data: BlockchainData) -> Result<impl warp::Reply, warp::Rejection> {
    let mut node = node.lock().await;
    let new_block = node.blockchain.write().await.add_block(data.data);
    let mut mew_chain_json = serde_json::to_string(&node.blockchain.read().await.chain).unwrap();
    mew_chain_json = "blockchain: ".to_string() + &mew_chain_json;
    node.event_sender.as_ref().unwrap().send(mew_chain_json).await
        .expect("Failed to send message to event sender");
    Ok(warp::reply::with_status(warp::reply::json(&new_block), StatusCode::CREATED))
}