use std::time::Instant;

use pnet::packet::tcp::TcpPacket;

use super::ip::L3Packet;


#[derive(Debug, PartialEq)]
pub struct FiveTupleWithFlagsAndTime {  // よく考えたらプロトコル番号がないので 5tuple ではない
    pub l3_src: String,
    pub l3_dst: String,
    pub l4_src: u16,
    pub l4_dst: u16,
    pub tcp_flags: u16,
    pub time: Instant,
}

impl FiveTupleWithFlagsAndTime {
    pub fn new(l3: &dyn L3Packet, l4: &TcpPacket) -> FiveTupleWithFlagsAndTime {
        FiveTupleWithFlagsAndTime {
            l3_src: l3.get_source(),
            l3_dst: l3.get_destination(),
            l4_src: l4.get_source(),
            l4_dst: l4.get_destination(),
            tcp_flags: l4.get_flags(),
            time: Instant::now(),
        }
    }
    pub fn is_same_src_and_dst(one: &FiveTupleWithFlagsAndTime, another: &FiveTupleWithFlagsAndTime) -> bool {
        return one.l3_src == another.l3_src
            && one.l3_dst == another.l3_dst
            && one.l4_src == another.l4_src
            && one.l4_dst == another.l4_dst
    }
    pub fn is_same_flow(one: &FiveTupleWithFlagsAndTime, another: &FiveTupleWithFlagsAndTime) -> bool {
        return(
                one.l3_src == another.l3_dst
            &&  one.l3_dst == another.l3_src
            &&  one.l4_src == another.l4_dst
            &&  one.l4_dst == another.l4_src
        )   || FiveTupleWithFlagsAndTime::is_same_src_and_dst(one, another);
    }
}