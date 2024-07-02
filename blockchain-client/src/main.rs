use clap::{Arg, Command};
use p2p::Node;

#[tokio::main]
async fn main() {
    let matches = Command::new("Blockchain client")
        .arg(Arg::new("http_port")
            .long("http_port")
            .value_name("PORT")
            .help("Sets a custom http port"))
        .arg(Arg::new("p2p_port")
            .long("p2p_port")
            .value_name("HOST")
            .help("Sets a custom p2p port"))
        .get_matches();

    let http_port = matches.get_one::<String>("http_port");
    let p2p_port = matches.get_one::<String>("p2p_port");

    if http_port.is_none() && p2p_port.is_none() {
        println!("Please provide at least one port");
        return;
    }

    let node = Node::new(http_port.unwrap().to_string(), p2p_port.unwrap().to_string());
    node.start().await.expect("Cannot start node");
}
