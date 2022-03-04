use log::{info};
use pnet::packet::{tcp::{TcpFlags, TcpPacket}, ipv6::Ipv6Packet};

// use self::ip::L3Packet;

pub mod ip;
pub mod tuples;

const WIDTH: usize = 20;


// pub fn print_packet_info(l3: &dyn L3Packet, l4: &TcpPacket) {
pub fn print_packet_info(l3: &Ipv6Packet, l4: &TcpPacket) {
    info!("{}", format!(
        "Captured a TCP packet from [{}]: {} to [{}]: {}, flags: {}\n",
        l3.get_source(),
        l4.get_source(),
        l3.get_destination(),
        l4.get_destination(),
        print_tcp_flags(l4)
    ));
    // println!("{}", "=".repeat(WIDTH * 3));
    // println!();
}

fn print_tcp_flags(l4: &TcpPacket) ->String {
    let mut res = String::new();
    if (l4.get_flags() & TcpFlags::URG) != 0 {
        res += &"URG, ".to_string();
    }

    if (l4.get_flags() & TcpFlags::ACK) != 0 {
        res += &"ACK, ".to_string();
    }

    if (l4.get_flags() & TcpFlags::PSH) != 0 {
        res += &"PSH, ".to_string();
    }

    if (l4.get_flags() & TcpFlags::RST) != 0 {
        res += &"RST, ".to_string();
    }

    if (l4.get_flags() & TcpFlags::SYN) != 0 {
        res += &"SYN, ".to_string();
    }

    if (l4.get_flags() & TcpFlags::FIN) != 0 {
        res += &"FIN, ".to_string();

    }
    // 末尾の ", " がダサいので削除
    res.pop(); res.pop();

    return res;
}