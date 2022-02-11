use crate::packet::ip::L3Packet;
use crate::packet::print_packet_info;
use crate::packet::tuples::FiveTupleWithFlagsAndTime;
use pnet::packet::tcp::{TcpPacket, TcpFlags};

// TCPパケットを構築する。
// 引数のパケットにはIPパケットが入り、それの皮を剥いたものを次に渡す。
pub fn tcp_handler(packet: &dyn L3Packet) -> Option<FiveTupleWithFlagsAndTime> {
    let tcp = TcpPacket::new(packet.get_payload());
    if let Some(tcp) = tcp {
        if (tcp.get_flags() & TcpFlags::ACK !=0) || (tcp.get_flags() & TcpFlags::SYN !=0){
            // print_packet_info(packet, &tcp);
            return Some(FiveTupleWithFlagsAndTime::new(packet, &tcp))
        }
    }
    None
}