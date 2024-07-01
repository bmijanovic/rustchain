mod types;
mod http_server;
mod p2p_server;

use libp2p::{gossipsub, mdns, Swarm};
use libp2p::swarm::NetworkBehaviour;
use architecture::blockchain::blockchain::Blockchain;
use serde::{Deserialize, Serialize};
use crate::http_server::server::run_server;
use crate::p2p_server::host::init_host;

#[derive(Serialize, Deserialize, Clone)]
pub struct Node{
    pub blockchain: Blockchain,
    pub host_port: String,
    pub p2p_port: String,
}

impl Node {
    pub fn new(host_port: String, p2p_port: String) -> Node {
        Node {
            blockchain: Blockchain::new(),
            host_port,
            p2p_port,
        }
    }

    pub async fn start(mut self) {
        println!("Starting blockchain client with p2p_port: {}, http_port: {}", self.p2p_port, self.host_port);
        let server = run_server(self.clone());
        let host = init_host(&self);
        tokio::join!(server, host);

    }
}

