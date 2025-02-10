use libp2p::{
    core::upgrade,
    identity,
    Multiaddr, PeerId, Transport,
    swarm::{Swarm, SwarmEvent},
};
use libp2p::noise::{NoiseConfig, X25519Spec, Keypair};
use libp2p::tcp::TokioTcpTransport;
use libp2p::yamux::YamuxConfig;
use libp2p::swarm::behaviour::toggle::Toggle;
use libp2p::ping::Ping;
use futures::prelude::*;
use tokio::time::Duration;

pub async fn start_p2p_node(listen_addr: Multiaddr) {
    // Generate a local identity.
    let local_key = identity::Keypair::generate_ed25519();
    let local_peer_id = PeerId::from(local_key.public());
    println!("Local Peer ID: {}", local_peer_id);

    // Create TCP transport using Tokio.
    let tcp_transport = TokioTcpTransport::default();

    // Set up Noise authentication.
    // Fully qualify the new() call to remove ambiguity.
    let noise_keys = <libp2p::noise::Keypair<X25519Spec>>::new();
    let noise_config = NoiseConfig::xx(
        noise_keys.into_authentic(&local_key)
            .expect("Failed to create authenticated noise keys"),
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

    // Instead of using Ping, we wrap it in a Toggle and disable it:
    let behaviour: Toggle<Ping> = Toggle::from(None);

    // Build the swarm with the dummy behaviour.
    let mut swarm = Swarm::new(transport, behaviour, local_peer_id);

    // Start listening.
    swarm.listen_on(listen_addr).expect("Failed to listen on address");

    // Process swarm events (only listening events, as no behaviour events are produced).
    while let Some(event) = swarm.next().await {
        match event {
            SwarmEvent::NewListenAddr { address, .. } => {
                println!("Listening on {}", address);
            }
            _ => {}
        }
    }
}
