use std::{time::Instant, net::Ipv6Addr};

use log::info;
use pnet::packet::{tcp::TcpPacket, ipv6::Ipv6Packet, ip::IpNextHeaderProtocols};

// use crate::schema::rtts::sid;




#[derive(Debug, PartialEq)]

pub struct FiveTupleWithFlagsAndTime {  // よく考えたらプロトコル番号がないので 5tuple ではない
    pub l3_src: String,
    pub l3_dst: String,
    pub l4_src: u16,
    pub l4_dst: u16,
    pub tcp_flags: u16,
    pub sid: Option<String>,
    pub time: Instant,
}

impl FiveTupleWithFlagsAndTime {
    // pub fn new(l3: &dyn L3Packet, l4: &TcpPacket) -> FiveTupleWithFlagsAndTime {
    pub fn new(l3: &Ipv6Packet, l4: &TcpPacket, sid: Option<Ipv6Addr>) -> FiveTupleWithFlagsAndTime {
        // let header = l3.get_next_header();
        // let sid = if header == IpNextHeaderProtocols::Ipv6Route {
        //     Some(l3.get_destination().to_string())
        // } else {
        //     None
        // };
        let sid_str = match sid {
            Some(sid) => Some(sid.to_string()),
            None => None,
        };
        info!("from 5tup, sid_str: {}", format!("{:?}", sid_str));
        FiveTupleWithFlagsAndTime {
            l3_src: l3.get_source().to_string(),
            l3_dst: l3.get_destination().to_string(),
            l4_src: l4.get_source(),
            l4_dst: l4.get_destination(),
            tcp_flags: l4.get_flags(),
            sid: sid_str,
            time: Instant::now(),
        }
    }
    pub fn create_key(&self) -> String {
        format!("[{}]:{},[{}]:{}", self.l3_src, self.l4_src, self.l3_dst, self.l4_dst)
    }
    pub fn reverse_create_key(&self) -> String {
        format!("[{}]:{},[{}]:{}", self.l3_dst, self.l4_dst, self.l3_src, self.l4_src)
    }
}