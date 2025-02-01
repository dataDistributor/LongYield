use libp2p::{
    core::upgrade,
    identity,
    Multiaddr, PeerId, Transport,
    swarm::{Swarm, SwarmEvent},
};
use libp2p_noise::{NoiseConfig, Keypair as NoiseKeypair, X25519Spec};
use libp2p_ping::{Ping, PingConfig, PingEvent};
use libp2p_tcp::{Config as TcpConfig, TokioTcpTransport};
use libp2p_yamux::YamuxConfig;
use libp2p_swarm_derive::NetworkBehaviour; // For the derive macro
use futures::prelude::*;
use tokio::time::Duration;

#[derive(NetworkBehaviour)]
#[behaviour(out_event = "MyBehaviourEvent")]
pub struct MyBehaviour {
    pub ping: Ping,
}

#[derive(Debug)]
pub enum MyBehaviourEvent {
    Ping(PingEvent),
}

impl From<PingEvent> for MyBehaviourEvent {
    fn from(event: PingEvent) -> Self {
        MyBehaviourEvent::Ping(event)
    }
}

pub async fn start_p2p_node(listen_addr: Multiaddr) {
    // Generate a local identity.
    let local_key = identity::Keypair::generate_ed25519();
    let local_peer_id = PeerId::from(local_key.public());
    println!("Local Peer ID: {}", local_peer_id);

    // Create TCP transport using Tokio.
    let tcp_config = TcpConfig::default().nodelay(true);
    let tcp_transport = TokioTcpTransport::new(tcp_config);

    // Set up Noise authentication.
    let noise_keys: NoiseKeypair<X25519Spec> = NoiseKeypair::new();
    // Bring into scope the trait for into_authentic.
    use libp2p_noise::AuthenticKeypair;
    let noise_config = NoiseConfig::xx(
        noise_keys
            .into_authentic(&local_key)
            .expect("Failed to create authenticated noise keys")
    )
    .into_authenticated();

    // Set up multiplexing with Yamux.
    let yamux_config = YamuxConfig::default();

    // Build the transport.
    let transport = tcp_transport
        .upgrade(upgrade::Version::V1)
        .authenticate(noise_config)
        .multiplex(yamux_config)
        .timeout(Duration::from_secs(20))
        .boxed();

    // Create a Ping behaviour.
    let ping_config = PingConfig::new();
    let behaviour = MyBehaviour {
        ping: Ping::new(ping_config),
    };

    // In libp2p 0.55, Swarm::new now requires a fourth argument: a Swarm configuration.
    // We use the configuration that disables the executor.
    let mut swarm = Swarm::new(
        transport,
        behaviour,
        local_peer_id,
        libp2p::swarm::Config::without_executor(),
    );

    // Start listening.
    swarm.listen_on(listen_addr).expect("Failed to listen on address");

    // Process swarm events.
    while let Some(event) = swarm.next().await {
        match event {
            SwarmEvent::NewListenAddr { address, .. } => println!("Listening on {}", address),
            SwarmEvent::Behaviour(MyBehaviourEvent::Ping(ev)) => println!("Ping event: {:?}", ev),
            _ => {}
        }
    }
}
