use std::error::Error;
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
use std::sync::Arc;

use iced_native::futures::channel::mpsc::{self, Receiver, Sender};
use iced_native::{subscription, Subscription};
use log::{info, trace};

pub use shared::{clipboard::*, mdns::*};
use std::net::UdpSocket;

pub const _SERVICE_NAME: &str = "_super_clipboard-sync._udp.local";
const DEFAULT_PORT: u16 = 50692;
const DEFAULT_MULTICAST: &str = "239.255.42.98";
const IP_ALL: [u8; 4] = [0, 0, 0, 0];

fn bind_multicast(
    addr: &SocketAddrV4,
    multi_addr: &SocketAddrV4,
) -> Result<std::net::UdpSocket, Box<dyn Error>> {
    use socket2::{Domain, Protocol, Socket, Type};

    assert!(multi_addr.ip().is_multicast(), "Must be multcast address");

    let socket = Socket::new(Domain::IPV4, Type::DGRAM, Some(Protocol::UDP))?;

    socket.set_reuse_address(true)?;
    socket.bind(&socket2::SockAddr::from(*addr))?;
    socket.set_multicast_loop_v4(true)?;
    socket.join_multicast_v4(multi_addr.ip(), addr.ip())?;

    Ok(socket.into())
}

pub fn create_socket() -> Arc<UdpSocket> {
    let addr = SocketAddrV4::new(IP_ALL.into(), DEFAULT_PORT);
    let multi_addr =
        SocketAddrV4::new(DEFAULT_MULTICAST.parse::<Ipv4Addr>().unwrap(), DEFAULT_PORT);

    info!("Starting server on: {}", addr);
    info!("Multicast address: {}\n", multi_addr);

    Arc::new(bind_multicast(&addr, &multi_addr).expect("Failed to bind multicast socket"))
}

pub fn start_sync(device: MDnsDevice) -> Subscription<Option<Event>> {
    struct Daemon;
    trace!("Starting Instantiate daemon");

    subscription::unfold(
        std::any::TypeId::of::<Daemon>(),
        State::Disconnected(device),
        |state| async move {
            match state {
                State::Disconnected(device) => {
                    let socket = create_socket();
                    let multi_addr = SocketAddrV4::new(
                        DEFAULT_MULTICAST.parse::<Ipv4Addr>().unwrap(),
                        DEFAULT_PORT,
                    );
                    let multi_addr: SocketAddr = multi_addr.into();
                    let (sender, receiver) = mpsc::channel(100);

                    socket
                        .send_to(&MDnsMessage::Connected { device }.to_bytes(), &multi_addr)
                        .unwrap();

                    (
                        Some(Event::Connected(sender)),
                        State::Connected(multi_addr, socket, receiver),
                    )
                }
                State::Connected(addr, socket, mut receiver) => {
                    let mut buff = [0u8; 4096];
                    if let Ok(n) = socket.recv(&mut buff) {
                        if n > 0 {
                            let msg = MDnsMessage::from_bytes(&buff[..n]).unwrap();
                            // send to application
                            // app_sender.send(msg).unwrap();
                            return (
                                Some(Event::Message(msg)),
                                State::Connected(addr, socket, receiver),
                            );
                        }
                    }

                    if let Ok(msg) = receiver.try_next().map(|r| r.unwrap()) {
                        let msg = msg.clone();
                        let udp_tx = socket.clone();
                        let mut bytes = msg.to_bytes();
                        // send to other devices
                        udp_tx.send_to(&mut bytes[..], &addr).unwrap();
                    }
                    (None, State::Connected(addr, socket, receiver))
                }
            }
        },
    )
}

#[derive(Debug)]
enum State {
    Disconnected(MDnsDevice),
    Connected(SocketAddr, Arc<UdpSocket>, Receiver<MDnsMessage>),
}

#[derive(Debug, Clone)]
pub enum Event {
    Connected(Sender<MDnsMessage>),
    Message(MDnsMessage),
}
