mod types;
mod http_server;
mod p2p_server;

use std::sync::Arc;
use libp2p::swarm::{NetworkBehaviour};
use architecture::blockchain::blockchain::Blockchain;
use architecture::wallet::wallet::Wallet;
use architecture::wallet::transaction_pool::TransactionPool;
use serde::{Deserialize, Serialize};
use tokio::sync::{mpsc, Mutex, RwLock};
use architecture::wallet::transaction::Transaction;
use crate::http_server::server::run_server;
use crate::p2p_server::host::{subscribe, build_swarm};
#[derive(Clone)]
pub struct Node{
    pub blockchain: Arc<RwLock<Blockchain>>,
    pub host_port: String,
    pub p2p_port: String,
    pub event_sender: Option<mpsc::Sender<String>>,
    pub wallet: Arc<RwLock<Wallet>>,
    pub transaction_pool: Arc<RwLock<TransactionPool>>
}

impl Node {
    pub fn new(host_port: String, p2p_port: String) -> Node {
        Node {
            blockchain: Arc::new(RwLock::new(Blockchain::new())),
            host_port,
            p2p_port,
            event_sender: None,
            wallet: Arc::new(RwLock::new(Wallet::new())),
            transaction_pool: Arc::new(RwLock::new(TransactionPool::new()))
        }
    }

    pub async fn start(mut self) -> Result<(), Box<dyn std::error::Error>>{
        println!("Starting blockchain client with p2p_port: {}, http_port: {}", self.p2p_port, self.host_port);
        let (event_sender, event_receiver) = mpsc::channel(100);
        self.event_sender = Some(event_sender.clone());
        let swarm = build_swarm()?;
        let p2p = subscribe(self.clone(), event_receiver, event_sender, swarm);
        let http = run_server(self.clone());
        tokio::join!(p2p, http);
        Ok(())
    }

    pub async fn mine(mut self) -> Result<(), Box<dyn std::error::Error>> {
        let mut valid_transactions = self.transaction_pool.read().await.valid_transactions();

        let wallet = self.wallet.read().await.clone();
        valid_transactions.push(Transaction::reward_transaction(&wallet, &Wallet::blockchain_wallet()));

        let mut blockchain = self.blockchain.write().await;
        blockchain.add_block(valid_transactions);

        let mut mew_chain_json = serde_json::to_string(&blockchain.chain).unwrap();
        mew_chain_json = "blockchain: ".to_string() + &mew_chain_json;
        self.event_sender.as_ref().unwrap().send(mew_chain_json).await
            .expect("Failed to send message to event sender");

        self.transaction_pool.write().await.transactions.clear();
        // broadcast to everyone to clear their transaction pools
        self.event_sender.as_ref().unwrap().send("transaction_pool_clear: clear".to_string()).await
            .expect("Failed to send message to event sender");

        Ok(())
    }
}
