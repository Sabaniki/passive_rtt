use std::net::Ipv6Addr;

// use crate::packet::ip::L3Packet;
use crate::packet::print_packet_info;
use crate::packet::tuples::FiveTupleWithFlagsAndTime;
use log::info;
use pnet::packet::{tcp::{TcpPacket, TcpFlags}, ipv6::Ipv6Packet, Packet};

// IP のペイロードからから TCP パケットを抽出して次に渡す。
// pub fn tcp_handler(packet: &dyn L3Packet) -> Option<FiveTupleWithFlagsAndTime> {
pub fn tcp_handler(packet: &Ipv6Packet, sid: Option<Ipv6Addr>) -> Option<FiveTupleWithFlagsAndTime> {
    let tcp = TcpPacket::new(packet.payload());
    if let Some(tcp) = tcp {
        print_packet_info(packet, &tcp);
        info!("from tcp_handler, sid: {}", format!("{:?}", sid));
        if (tcp.get_flags() & TcpFlags::ACK !=0) || (tcp.get_flags() & TcpFlags::SYN !=0){
            return Some(FiveTupleWithFlagsAndTime::new(packet, &tcp, sid))
        }
    }
    None
}