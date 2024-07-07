use futures::stream::StreamExt;
use libp2p::{gossipsub, mdns, noise, PeerId, Swarm, swarm::NetworkBehaviour, swarm::SwarmEvent, tcp, yamux};
use std::collections::hash_map::DefaultHasher;
use std::error::Error;
use std::hash::{Hash, Hasher};
use std::time::Duration;
use libp2p::gossipsub::IdentTopic;
use tokio::{select};
use tokio::sync::mpsc::{Receiver};
use tracing_subscriber::EnvFilter;
use architecture::blockchain::block::Block;
use architecture::wallet::transaction::Transaction;
use crate::Node;

#[derive(NetworkBehaviour)]
pub(crate) struct MyBehaviour {
    pub(crate) gossipsub: gossipsub::Behaviour,
    mdns: mdns::tokio::Behaviour,
}




pub async fn subscribe(mut node: Node, mut event_receiver: Receiver<String>, mut swarm: Swarm<MyBehaviour>) -> Result<(), Box<dyn Error>> {

    // swarm.listen_on("/ip4/0.0.0.0/udp/0/quic-v1".parse()?)?;
    swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;
    // swarm.listen_on("/ip4/localhost/tcp/0".parse()?)?;

    let blockchain_topic = IdentTopic::new("blockchain");
    swarm.behaviour_mut().gossipsub.subscribe(&blockchain_topic)?;

    let transaction_pool_topic = IdentTopic::new("transaction_pool");
    swarm.behaviour_mut().gossipsub.subscribe(&transaction_pool_topic)?;

    let transaction_pool_clear_topic = IdentTopic::new("transaction_pool_clear");
    swarm.behaviour_mut().gossipsub.subscribe(&transaction_pool_clear_topic)?;

    //sleep for 4 seconds to allow the server to start
    tokio::time::sleep(tokio::time::Duration::from_secs(4)).await;

    loop {
        select! {
            event = swarm.select_next_some() => {
                handle_event(&mut swarm, event, &mut node).await;
            }
            Some(data) = event_receiver.recv() => {
                process_input(&mut swarm, &blockchain_topic, &transaction_pool_topic, &transaction_pool_clear_topic, data);
            }
        }
    }
}

pub(crate) fn send_message(swarm: &mut Swarm<MyBehaviour>, topic: &IdentTopic, message: String) {
    if let Err(e) = swarm.behaviour_mut().gossipsub.publish(topic.clone(), message.as_bytes()) {
        println!("Publish error: {e:?}");
    }
}


fn process_input(swarm: &mut Swarm<MyBehaviour>, blockchain_topic: &IdentTopic, transaction_pool_topic: &IdentTopic, transaction_pool_clear_topic: &IdentTopic, line: String) {
    let mut parts = line.splitn(2, ": ");
    if let (Some(topic_str), Some(message)) = (parts.next(), parts.next()) {
        let topic = match topic_str {
            "blockchain" => blockchain_topic,
            "transaction_pool" => transaction_pool_topic,
            "transaction_pool_clear" => transaction_pool_clear_topic,
            _ => {
                println!("Unknown topic: {topic_str}");
                return;
            }
        };
        send_message(swarm, topic, message.to_string());
    } else {
        println!("Invalid message format. Use '<topic>: <message>'");
    }
}

async fn handle_event(swarm: &mut Swarm<MyBehaviour>, event: SwarmEvent<MyBehaviourEvent>, node: &mut Node) {
    match event {
        SwarmEvent::Behaviour(MyBehaviourEvent::Mdns(mdns::Event::Discovered(list))) => {
            for (peer_id, _multiaddr) in list {
                println!("mDNS discovered a new peer: {peer_id}");
                swarm.behaviour_mut().gossipsub.add_explicit_peer(&peer_id);
            }

        },
        SwarmEvent::Behaviour(MyBehaviourEvent::Mdns(mdns::Event::Expired(list))) => {
            for (peer_id, _multiaddr) in list {
                println!("mDNS discover peer has expired: {peer_id}");
                swarm.behaviour_mut().gossipsub.remove_explicit_peer(&peer_id);
            }
        },
        SwarmEvent::Behaviour(MyBehaviourEvent::Gossipsub(gossipsub::Event::Message {
                                                              propagation_source: peer_id,
                                                              message_id: id,
                                                              message,
                                                          })) => {
            match_topic_message(&message.topic.as_str(), &String::from_utf8_lossy(&message.data), id, &peer_id, node).await;
        },
        SwarmEvent::NewListenAddr { address, .. } => {
            println!("Local node is listening on {address}");
        }
        SwarmEvent::Behaviour(MyBehaviourEvent::Gossipsub(gossipsub::Event::Subscribed { peer_id, topic })) => {
            println!("Subscribed to '{topic}' from {peer_id}");
            if topic.as_str() == "blockchain" {
                send_message(swarm, &IdentTopic::new("blockchain"), serde_json::to_string(&node.blockchain.read().await.chain).unwrap());
            }

            // send_message(swarm, &IdentTopic::new("blockchain"), serde_json::to_string(&node.blockchain.read().await.chain).unwrap());
        }
        other => {
            println!("Other event: {:?}", other);
        }
    }
}

async fn match_topic_message(topic: &str, msg: &str, id: gossipsub::MessageId, peer_id: &PeerId, node: &mut Node) {
    match topic {
        "blockchain" => {
            println!("Received blockchain message: '{msg}' with id: {id} from peer: {peer_id}");
            let new_chain = serde_json::from_str::<Vec<Block>>(&msg).unwrap();
            for block in &new_chain {
                println!("{block}");
            }
            node.blockchain.write().await.replace_chain(new_chain);

        },
        "transaction_pool" => {
            println!("Received transaction_pool message: '{msg}' with id: {id} from peer: {peer_id}");
            let transaction = serde_json::from_str::<Transaction>(&msg).unwrap();
            println!("{transaction}");
            node.transaction_pool.write().await.update_or_add_transaction(transaction);
        },
        "transaction_pool_clear" => {
            println!("Received transaction_pool_clear message: '{msg}' with id: {id} from peer: {peer_id}");
            node.transaction_pool.write().await.transactions.clear();
        },
        _ => {
            println!("Received message on unknown topic '{topic}': '{msg}' with id: {id} from peer: {peer_id}");
        }
    }
}

pub fn build_swarm() -> Result<libp2p::Swarm<MyBehaviour>, Box<dyn Error>> {
    let _ = tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .try_init();
    let swarm = libp2p::SwarmBuilder::with_new_identity()
        .with_tokio()
        .with_tcp(
            tcp::Config::default(),
            noise::Config::new,
            yamux::Config::default,
        )?
        .with_quic()
        .with_behaviour(|key| {
            let message_id_fn = |message: &gossipsub::Message| {
                let mut s = DefaultHasher::new();
                message.data.hash(&mut s);
                gossipsub::MessageId::from(s.finish().to_string())
            };

            let gossipsub_config = gossipsub::ConfigBuilder::default()
                .heartbeat_interval(Duration::from_secs(1))
                .validation_mode(gossipsub::ValidationMode::Permissive)
                .mesh_n(12)  // Increased number of peers in mesh
                .mesh_n_low(6)
                .duplicate_cache_time(Duration::from_secs(0))
                .message_id_fn(message_id_fn)
                .build()
                .map_err(|msg| std::io::Error::new(std::io::ErrorKind::Other, msg))?;

            let gossipsub = gossipsub::Behaviour::new(
                gossipsub::MessageAuthenticity::Signed(key.clone()),
                gossipsub_config,
            )?;

            let mdns =
                mdns::tokio::Behaviour::new(mdns::Config::default(), key.public().to_peer_id())?;
            Ok(MyBehaviour { gossipsub, mdns })
        })?
        .with_swarm_config(|c| c.with_idle_connection_timeout(Duration::from_secs(60)))
        .build();

    Ok(swarm)
}



