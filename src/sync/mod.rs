mod mdns;

use std::error::Error;
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
use std::sync::Arc;

use iced_native::futures::{
    channel::mpsc::{self, Receiver, Sender},
    SinkExt,
};
use iced_native::{subscription, Subscription};
use log::{info, trace};

pub use mdns::*;
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

    // let std_socket = bind_multicast(&addr, &multi_addr).expect("Failed to bind multicast socket");
    Arc::new(bind_multicast(&addr, &multi_addr).expect("Failed to bind multicast socket"))
    // let socket = UdpSocket::from_std(std_socket).unwrap();
    // Arc::new(socket)
}

pub fn start_sync() -> Subscription<Event> {
    struct Daemon;
    trace!("Starting Instantiate daemon");

    subscription::channel(
        std::any::TypeId::of::<Daemon>(),
        100,
        |mut output| async move {
            let mut state = State::Disconnected;

            loop {
                match &mut state {
                    State::Disconnected => {
                        let socket = create_socket();
                        let multi_addr = SocketAddrV4::new(
                            DEFAULT_MULTICAST.parse::<Ipv4Addr>().unwrap(),
                            DEFAULT_PORT,
                        );
                        let (sender, receiver) = mpsc::channel(100);

                        output.send(Event::Connected(sender)).await.unwrap();
                        state = State::Connected(multi_addr.into(), socket, receiver);
                    }
                    State::Connected(addr, socket, receiver) => {
                        let mut buff = [0u8; 4096];
                        if let Ok(n) = socket.recv(&mut buff) {
                            if n == 0 {
                                continue;
                            }
                            let msg = MDnsMessage::from_bytes(&buff[..n]).unwrap();
                            // send to application
                            // app_sender.send(msg).unwrap();
                            output.send(Event::Message(msg)).await.unwrap();
                        }

                        if let Ok(msg) = receiver.try_next().map(|r| r.unwrap()) {
                            let msg = msg.clone();
                            let udp_tx = socket.clone();
                            let mut bytes = msg.to_bytes();
                            // send to other devices
                            udp_tx.send_to(&mut bytes[..], &*addr).unwrap();
                        }
                    }
                }
            }
        },
    )

    // match state {
    //     State::Disconnected(app_sender, app_receiver) => {
    //         let socket = create_socket();
    //         let multi_addr = SocketAddrV4::new(
    //             DEFAULT_MULTICAST.parse::<Ipv4Addr>().unwrap(),
    //             DEFAULT_PORT,
    //         );
    //
    //         let (sync_sender, sync_receiver) = mpsc::channel::<MDnsMessage>();
    //
    //         // receive from other devices
    //         {
    //             let socket = socket.clone();
    //             std::thread::spawn(move || {
    //                 let mut buff = [0u8; 4096];
    //
    //                 while let Ok(n) = socket.recv(&mut buff) {
    //                     if n == 0 {
    //                         continue;
    //                     }
    //                     let msg = MDnsMessage::from_bytes(&buff).unwrap();
    //                     // send to application
    //                     // app_sender.send(msg).unwrap();
    //                     sync_sender.send(msg).unwrap();
    //                 }
    //             });
    //         }
    //
    //         (
    //             Some(Event::Connected),
    //             State::Connected(multi_addr.into(), sync_receiver, socket),
    //         )
    //     }
    //     // sender to other devices
    //     // when app_conn receive data from application
    //     // Receiver from application
    //     State::Connected(addr, receiver, udp_tx) => match receiver.try_recv() {
    //         Ok(msg) => {
    //             let msg = msg.clone();
    //             let udp_tx = udp_tx.clone();
    //             let mut bytes = msg.to_bytes();
    //             // send to other devices
    //             udp_tx.send_to(&mut bytes[..], &addr).unwrap();
    //             (
    //                 Some(Event::Message(msg)),
    //                 State::Connected(addr, receiver, udp_tx),
    //             )
    //         }
    //         Err(_) => (None, State::Connected(addr, receiver, udp_tx)),
    //     },
    // }
}

#[derive(Debug)]
enum State {
    Disconnected,
    Connected(SocketAddr, Arc<UdpSocket>, Receiver<MDnsMessage>),
}

#[derive(Debug, Clone)]
pub enum Event {
    Connected(Sender<MDnsMessage>),
    Message(MDnsMessage),
}
