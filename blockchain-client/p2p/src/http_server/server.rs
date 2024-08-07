use std::sync::Arc;
use tokio::sync::{Mutex};
use crate::Node;
use warp::{http::Method, Filter, Reply};
use crate::http_server::routes;

pub async fn run_server(node: Node) {
    let node = Arc::new(Mutex::new(node));
    let routes = build_routes(node.clone()).await;
    let host_port: u16 = node.lock().await.host_port.parse().unwrap();
    warp::serve(routes).run(([0, 0, 0, 0], host_port)).await;
}

async fn build_routes(node: Arc<Mutex<Node>>) -> impl Filter<Extract = impl Reply> + Clone {
    let node_filter = warp::any().map(move || node.clone());
    let cors = warp::cors()
        .allow_any_origin()
        .allow_header("content-type")
        .allow_methods(&[
            Method::PUT,
            Method::DELETE,
            Method::GET,
            Method::POST,
        ]);

    let hello = warp::get()
        .and(warp::path("hello"))
        .and(warp::path::end())
        .and_then(routes::hello_world);

    let blockchain = warp::get()
        .and(warp::path("blockchain"))
        .and(warp::path::end())
        .and(node_filter.clone())
        .and_then(routes::print_blockchain);

    let mine_block = warp::post()
        .and(warp::path("mine"))
        .and(warp::path::end())
        .and(node_filter.clone())
        .and_then(routes::mine_block);

    let print_transactions = warp::get()
        .and(warp::path("transactions"))
        .and(warp::path::end())
        .and(node_filter.clone())
        .and_then(routes::print_transactions);

    let post_transaction = warp::post()
        .and(warp::path("transaction"))
        .and(warp::path::end())
        .and(node_filter.clone())
        .and(warp::body::json())
        .and_then(routes::post_transaction);

    let public_key = warp::get()
        .and(warp::path("public_key"))
        .and(warp::path::end())
        .and(node_filter.clone())
        .and_then(routes::get_public_key);

    let wallet_balance = warp::get()
        .and(warp::path("balance"))
        .and(warp::path::end())
        .and(node_filter.clone())
        .and_then(routes::get_wallet_balance);

    hello
        .or(blockchain)
        .or(mine_block)
        .or(print_transactions)
        .or(post_transaction)
        .or(public_key)
        .or(wallet_balance)
        .with(cors)
        .with(warp::trace::request())
}