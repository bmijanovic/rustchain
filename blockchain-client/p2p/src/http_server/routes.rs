use std::sync::{Arc, Mutex};
use libp2p::gossipsub::IdentTopic;
use warp::http::StatusCode;
use crate::Node;
use crate::types::dto::BlockchainData;

pub async fn hello_world() -> Result<impl warp::Reply, warp::Rejection> {
    Ok(warp::reply::json(&"Hello, world!"))
}

// print the blockchain from node
pub async fn print_blockchain(node: Arc<Mutex<Node>>) -> Result<impl warp::Reply, warp::Rejection> {
    let node = node.lock().unwrap();
    Ok(warp::reply::with_status(warp::reply::json(&node.blockchain), StatusCode::OK))

}

pub async fn mine_block(node: Arc<Mutex<Node>>, data: BlockchainData) -> Result<impl warp::Reply, warp::Rejection> {
    println!("Trying to lock");
    let mut node = node.lock().unwrap();
    let new_block = node.blockchain.add_block(data.data);
    let mut swarm = node.swarm.lock().unwrap();
    println!("Locked");
    let blockchain_topic = IdentTopic::new("blockchain");
    println!("Publishing to blockchain topic");
    if let Err(e) = swarm.behaviour_mut().gossipsub.publish(blockchain_topic.clone(), "asdasd".to_string().as_bytes()) {
        println!("Publish error: {e:?}");
    }
    Ok(warp::reply::with_status(warp::reply::json(&new_block), StatusCode::CREATED))
}