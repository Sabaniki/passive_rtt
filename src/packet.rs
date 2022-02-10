use log::{info};
use pnet::packet::tcp::{TcpFlags, TcpPacket};

use self::ip::L3Packet;
// use self::transport::L4Packet;

pub mod ip;
pub mod transport;

const WIDTH: usize = 20;


pub fn print_packet_info(l3: &dyn L3Packet, l4: &TcpPacket) {
    info!("{}", format!(
        "Captured a TCP packet from [{}]: {} to [{}]: {}, flag: {}\n",
        l3.get_source(),
        l4.get_source(),
        l3.get_destination(),
        l4.get_destination(),
        l4.get_flags()
    ));
    if (l4.get_flags() & TcpFlags::SYN) != 0 {
        info!("this packet is flagged 'SYN'")
    }

    if (l4.get_flags() & TcpFlags::ACK) != 0 {
        info!("this packet is flagged 'ACK'")
    }

    println!("{}", "=".repeat(WIDTH * 3));
    println!();
}
