use log::{info, debug};
use pnet::packet::ethernet::EthernetPacket;
use pnet::packet::ip::{IpNextHeaderProtocols, IpNextHeaderProtocol};
use pnet::packet::ipv4::Ipv4Packet;
use pnet::packet::ipv6::Ipv6Packet;
use pnet::packet::Packet;
use crate::handler::transport::tcp_handler;
// use crate::packet::ip::L3Packet;
use crate::packet::tuples::FiveTupleWithFlagsAndTime;

// Ether のペイロードから IPv6 パケットを抽出．次のレイヤのハンドラを呼び出す
pub fn v6_handler(ethernet: &EthernetPacket) -> Option<FiveTupleWithFlagsAndTime> {
    if let Some(packet) = Ipv6Packet::new(ethernet.payload()) {
        return call_transport_handler(&packet, packet.get_next_header());
    }
    None
}

fn call_transport_handler(packet: &Ipv6Packet ,next: IpNextHeaderProtocol) -> Option<FiveTupleWithFlagsAndTime> {
    debug!("call_taransport");
    match next {
        IpNextHeaderProtocols::Tcp => {
            tcp_handler(packet, None)
        },
        IpNextHeaderProtocols::Ipv6Route => {
            debug!("it includes sid");
            info!("from caller, sid: {}", format!("{:?}", Some(packet.get_destination())));
            // debug!("packet: {}", format!("{:?}", packet.packet().iter().map(|x| format!("{:02X}", x)).collect::<String>()));
            if let Some(outer) = Ipv6Packet::new(&packet.payload()) {
                // debug!("outer: {}", format!("{:?}", &outer.packet().iter().map(|x| format!("{:02X}", x)).collect::<String>()));
                // debug!("outer length: {}", format!("{:?}", &outer.packet().iter().len()));
                // debug!("outer after SRH: {}", format!("{:?}", &outer.packet()[24..].iter().map(|x| format!("{:02X}", x)).collect::<String>()));
                 // Length が 2 のときに 24 byte だったから (2 + 1) * 8 かなと解釈したけど正直良くわからない
                let aflter_srh: usize = ((&outer.packet()[1] + 1) * 8).into();
                if let Some(inner) = Ipv6Packet::new(&outer.packet()[aflter_srh..]){
                    // debug!("inner: {}", format!("{:?}", &inner.packet().iter().map(|x| format!("{:02X}", x)).collect::<String>()));
                    return tcp_handler(&inner, Some(packet.get_destination()))
                }
            }
            None
        }
        _ => {
            debug!("next header is not tcp and Ipv6Route");
            debug!("{}", format!("{:?}", packet.packet().iter().map(|x| format!("{:02X}", x)).collect::<String>()));
            None
        }
    }
}