use architecture::blockchain::blockchain::Blockchain;
mod routes;

#[derive(Clone)]
pub struct Node{
    pub blockchain: Blockchain,
    pub host_port: String,
    pub p2p_port: String
}

impl Node {
    pub fn new(host_port: String, p2p_port: String) -> Node {
        Node {
            blockchain: Blockchain::new(),
            host_port,
            p2p_port
        }
    }

    pub fn start(&self) {
        println!("Starting blockchain client with p2p_port: {}, http_port: {}", self.p2p_port, self.host_port);
        tokio::runtime::Builder::new_multi_thread()
            .worker_threads(2)
            .enable_all()
            .build()
            .unwrap()
            .block_on(routes::server::run_server(self.clone()));
    }
}

