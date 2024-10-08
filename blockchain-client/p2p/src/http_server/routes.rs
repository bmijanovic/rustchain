use std::ops::DerefMut;
use std::sync::Arc;

use tokio::sync::Mutex;
use warp::http::StatusCode;

use crate::Node;
use crate::types::dto::{Balance, PublicKey, TransactionData};

pub async fn hello_world() -> Result<impl warp::Reply, warp::Rejection> {
    Ok(warp::reply::json(&"Hello, world!"))
}

pub async fn print_blockchain(node: Arc<Mutex<Node>>) -> Result<impl warp::Reply, warp::Rejection> {
    let node = node.lock().await;
    let blockchain = node.blockchain.read().await.clone();
    Ok(warp::reply::with_status(warp::reply::json(&blockchain), StatusCode::OK))
}


pub async fn mine_block(node: Arc<Mutex<Node>>) -> Result<impl warp::Reply, warp::Rejection> {
    let node = node.lock().await;
    node.clone().mine().await.expect("Failed to mine block");
    Ok(warp::reply::with_status(warp::reply(), StatusCode::OK))
}

pub async fn print_transactions(node: Arc<Mutex<Node>>) -> Result<impl warp::Reply, warp::Rejection> {
    let node = node.lock().await;
    let transactions = &node.transaction_pool.read().await.transactions;
    Ok(warp::reply::with_status(warp::reply::json(&transactions), StatusCode::OK))
}

pub async fn post_transaction(node: Arc<Mutex<Node>>, data: TransactionData) -> Result<impl warp::Reply, warp::Rejection> {
    let node = node.lock().await;
    let mut wallet = node.wallet.write().await.clone();
    let blockchain = node.blockchain.read().await.clone();
    let transaction = wallet.create_transaction(data.recipient, data.amount,
                                                &mut node.transaction_pool.write().await.deref_mut(), &blockchain).unwrap();

    let mut transaction_json = serde_json::to_string(&transaction).unwrap();
    transaction_json = "transaction_pool: ".to_string() + &transaction_json;
    node.event_sender.as_ref().unwrap().send(transaction_json).await
        .expect("Failed to send message to event sender");
    Ok(warp::reply::with_status(warp::reply::json(&transaction), StatusCode::CREATED))
}

pub async fn get_public_key(node: Arc<Mutex<Node>>) -> Result<impl warp::Reply, warp::Rejection> {
    let node = node.lock().await;
    let wallet = node.wallet.read().await.clone();
    let pk = PublicKey {
        public_key: wallet.public_key.clone()
    };
    Ok(warp::reply::with_status(warp::reply::json(&pk), StatusCode::OK))
}

pub async fn get_wallet_balance(node: Arc<Mutex<Node>>) -> Result<impl warp::Reply, warp::Rejection> {
    let node = node.lock().await;
    let wallet = node.wallet.read().await.clone();
    let blockchain = node.blockchain.read().await.clone();
    let balance = wallet.calculate_balance(&blockchain);
    let balance = Balance {
        balance
    };
    Ok(warp::reply::with_status(warp::reply::json(&balance), StatusCode::OK))
}