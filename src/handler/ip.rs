use pnet::packet::ethernet::EthernetPacket;
use pnet::packet::ip::{IpNextHeaderProtocols, IpNextHeaderProtocol};
use pnet::packet::ipv4::Ipv4Packet;
use pnet::packet::ipv6::Ipv6Packet;
use pnet::packet::Packet;
use crate::handler::transport::{tcp_handler, udp_handler};
use crate::packet::ip::L3Packet;

// IPv4パケットを構築し、次のレイヤのハンドラを呼び出す
pub fn v4_handler(ethernet: &EthernetPacket) {
    if let Some(packet) = Ipv4Packet::new(ethernet.payload()) {
        call_transport_handler(&packet, packet.get_next_level_protocol());
    }
}

// IPv6パケットを構築し、次のレイヤのハンドラを呼び出す
pub fn v6_handler(ethernet: &EthernetPacket) {
    if let Some(packet) = Ipv6Packet::new(ethernet.payload()) {
        call_transport_handler(&packet, packet.get_next_header());
    }
}

// fn call_transport_handler(packet: &dyn GettableEndPoints,next: IpNextHeaderProtocol) {
//     match next {
//         IpNextHeaderProtocols::Tcp => {
//             tcp_handler(packet);
//         },
//         IpNextHeaderProtocols::Udp => {
//             udp_handler(packet);
//         },
//         _ => {
//             info!("This packet is neither TCP nor UDP");
//         }
//     }
// }

fn call_transport_handler(packet: &dyn L3Packet ,next: IpNextHeaderProtocol) {
    match next {
        IpNextHeaderProtocols::Tcp => {
            tcp_handler(packet);
        },
        IpNextHeaderProtocols::Udp => {
            udp_handler(packet);
        },
        _ => {
            info!("This packet is neither TCP nor UDP");
        }
    }
}