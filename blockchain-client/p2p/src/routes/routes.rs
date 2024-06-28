use std::sync::Arc;
// make hello world route
use warp::http::StatusCode;
use crate::Node;

pub async fn hello_world() -> Result<impl warp::Reply, warp::Rejection> {
    Ok(warp::reply::json(&"Hello, world!"))
}

// print the blockchain from node
pub async fn print_blockchain(node: Arc<Node>) -> Result<impl warp::Reply, warp::Rejection> {
    Ok(warp::reply::json(&node.blockchain))
}