use pnet::packet::ethernet::EthernetPacket;
use pnet::packet::ip::{IpNextHeaderProtocols, IpNextHeaderProtocol};
use pnet::packet::ipv4::Ipv4Packet;
use pnet::packet::ipv6::Ipv6Packet;
use pnet::packet::Packet;
use crate::handler::transport::tcp_handler;
use crate::packet::ip::L3Packet;
use crate::packet::tuples::FiveTupleWithFlagsAndTime;

// IPv4パケットを構築し、次のレイヤのハンドラを呼び出す
pub fn v4_handler(ethernet: &EthernetPacket) -> Option<FiveTupleWithFlagsAndTime> {
    if let Some(packet) = Ipv4Packet::new(ethernet.payload()) {
        return call_transport_handler(&packet, packet.get_next_level_protocol());
    }
    None
}

// IPv6パケットを構築し、次のレイヤのハンドラを呼び出す
pub fn v6_handler(ethernet: &EthernetPacket) -> Option<FiveTupleWithFlagsAndTime> {
    if let Some(packet) = Ipv6Packet::new(ethernet.payload()) {
        return call_transport_handler(&packet, packet.get_next_header());
    }
    None
}

fn call_transport_handler(packet: &dyn L3Packet ,next: IpNextHeaderProtocol) -> Option<FiveTupleWithFlagsAndTime> {
    match next {
        IpNextHeaderProtocols::Tcp => {
            tcp_handler(packet)
        },
        _ => {
            None
        }
    }
}