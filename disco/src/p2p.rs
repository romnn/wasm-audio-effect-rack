use libp2p::{
    core::upgrade,
    floodsub::{Floodsub, FloodsubEvent, Topic},
    identity,
    mdns::{Mdns, MdnsEvent},
    mplex,
    noise::{Keypair, NoiseConfig, X25519Spec},
    swarm::{NetworkBehaviourEventProcess, Swarm, SwarmBuilder},
    tcp::TokioTcpConfig,
    NetworkBehaviour, PeerId, Transport,
};
use log::{error, info};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use tokio::{fs, io::AsyncBufReadExt, sync::mpsc};

// static KEYS: Lazy<identity::Keypair> = Lazy::new(|| identity::Keypair::generate_ed25519());
// static PEER_ID: Lazy<PeerId> = Lazy::new(|| PeerId::from(KEYS.public()));
// static TOPIC: Lazy<Topic> = Lazy::new(|| Topic::new("recipes"));

fn p2p_test() -> () {
    let key_pair = identity::Keypair::generate_ed25519();
    let peer_id = PeerId::from(key_pair.public());
    println!("peer id: {:?}", peer_id);

    // let (response_sender, mut response_rcv) = mpsc::unbounded_channel();

    let auth_keys = Keypair::<X25519Spec>::new()
        .into_authentic(&key_pair)
        .expect("can create auth keys");
}
