mod types;
mod http_server;
mod p2p_server;

use std::sync::{Arc, Mutex};
use libp2p::{Swarm};
use libp2p::swarm::NetworkBehaviour;
use architecture::blockchain::blockchain::Blockchain;
use serde::{Deserialize, Serialize};
use crate::http_server::server::run_server;
use crate::p2p_server::host::{subscribe, build_swarm};
use crate::p2p_server::host::MyBehaviour;

#[derive(Clone)]
pub struct Node{
    pub blockchain: Blockchain,
    pub host_port: String,
    pub p2p_port: String,
    pub swarm: Arc<Mutex<Swarm<MyBehaviour>>>

}

impl Node {
    pub fn new(host_port: String, p2p_port: String) -> Node {
        Node {
            blockchain: Blockchain::new(),
            host_port,
            p2p_port,
            swarm: Arc::new(Mutex::from(build_swarm().unwrap()))
        }
    }

    pub async fn start(self) {
        println!("Starting blockchain client with p2p_port: {}, http_port: {}", self.p2p_port, self.host_port);
        let p2p = subscribe(self.clone());
        let http = run_server(self.clone());
        tokio::join!(p2p, http);
    }
}

