use std::sync::{Arc, Mutex};
use crate::Node;
use warp::{http::Method, Filter, Reply};
use crate::http_server::routes;

pub async fn run_server(node: Node) {
    let node_arc = Arc::new(Mutex::new(node));
    let routes = build_routes(node_arc.clone()).await;
    let host_port: u16 = node_arc.lock().unwrap().host_port.parse().unwrap();
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
        .and(warp::body::json())
        .and_then(routes::mine_block);

    hello
        .or(blockchain)
        .or(mine_block)
        .with(cors)
        .with(warp::trace::request())
}