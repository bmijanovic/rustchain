use std::sync::Arc;
use crate::Node;
use warp::{http::Method, Filter, Reply};
use crate::routes::routes;

pub async fn run_server(node: Node) {
    let node_arc = Arc::new(node);
    let routes = build_routes(node_arc.clone()).await;
    let host_port: u16 = node_arc.host_port.parse().unwrap();
    warp::serve(routes).run(([0, 0, 0, 0], host_port)).await;
}

async fn build_routes(node: Arc<Node>) -> impl Filter<Extract = impl Reply> + Clone {
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
        .and_then(move || routes::print_blockchain(node.clone()));



    hello
        .or(blockchain)
        .with(cors)
}