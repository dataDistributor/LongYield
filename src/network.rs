use libp2p::{
    core::upgrade,
    identity,
    noise,
    ping,
    swarm::{Swarm, SwarmEvent, SwarmBuilder},
    tcp,
    yamux,
    Multiaddr,
    PeerId,
    Transport,
    futures::StreamExt,
};
use tokio::runtime::Handle;

pub async fn start_p2p_node(listen_addr: Multiaddr) {
    // Generate a local Ed25519 keypair and derive the peer ID.
    let local_key = identity::Keypair::generate_ed25519();
    let local_peer_id = PeerId::from(local_key.public());

    // --- Transport Setup ---

    // Create a TCP transport.
    let tcp_transport = tcp::TcpConfig::new();

    // Prepare the noise authentication.
    // First, derive the Noise keypair from the local key.
    let noise_keys = noise::Keypair::<noise::X25519Spec>::new()
        .into_authentic(&local_key)
        .expect("Failed to create authentic Noise keys");

    // Build the transport by upgrading, authenticating via Noise, and multiplexing with Yamux.
    let transport = tcp_transport
        .upgrade(upgrade::Version::V1)
        .authenticate(
            noise::NoiseConfig::xx(noise_keys)
                .into_authenticated(),
        )
        .multiplex(yamux::YamuxConfig::default())
        .boxed();

    // --- Behavior Setup ---

    // Create a simple ping behavior.
    let ping_behavior = ping::Behaviour::new(ping::Config::new());

    // --- Swarm Setup ---

    // Build the swarm using SwarmBuilder.
    // In libp2p-swarm v0.34.0, Swarm::new takes 3 arguments.
    // To specify an executor, use the builder’s `.executor(...)` method.
    let mut swarm = SwarmBuilder::new(transport, ping_behavior, local_peer_id)
        .executor(Box::new(|fut| {
            // Spawn the future onto the current Tokio runtime.
            Handle::current().spawn(fut);
        }))
        .build();

    // Start listening on the provided multiaddress.
    swarm.listen_on(listen_addr).expect("Failed to start listener");
    println!("Peer ID: {}", swarm.local_peer_id());

    // Process swarm events in a loop.
    loop {
        match swarm.select_next_some().await {
            SwarmEvent::NewListenAddr { address, .. } => {
                println!("Listening on {}", address);
            }
            SwarmEvent::Behaviour(event) => {
                println!("Ping event: {:?}", event);
            }
            _ => {}
        }
    }
}
