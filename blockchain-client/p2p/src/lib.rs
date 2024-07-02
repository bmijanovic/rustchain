mod types;
mod http_server;
mod p2p_server;

use std::sync::Arc;
use tokio::sync::{Mutex};
use libp2p::{Swarm};
use libp2p::swarm::{NetworkBehaviour, SwarmEvent};
use architecture::blockchain::blockchain::Blockchain;
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;
use crate::http_server::server::run_server;
use crate::p2p_server::host::{subscribe, build_swarm, MyBehaviourEvent};
use crate::p2p_server::host::MyBehaviour;

#[derive(Clone)]
pub struct Node{
    pub blockchain: Blockchain,
    pub host_port: String,
    pub p2p_port: String,
    pub swarm: Arc<Mutex<Swarm<MyBehaviour>>>,
    pub event_sender: Option<mpsc::Sender<String>>
}

impl Node {
    pub fn new(host_port: String, p2p_port: String) -> Node {
        Node {
            blockchain: Blockchain::new(),
            host_port,
            p2p_port,
            swarm: Arc::new(Mutex::from(build_swarm().unwrap())),
            event_sender: None
        }
    }

    pub async fn start(mut self) {
        println!("Starting blockchain client with p2p_port: {}, http_port: {}", self.p2p_port, self.host_port);
        let (event_sender, mut event_receiver) = mpsc::channel(100);
        self.event_sender = Some(event_sender.clone());
        let p2p = subscribe(self.clone(), event_receiver, event_sender);
        let http = run_server(self.clone());
        tokio::join!(p2p, http);
    }
}
