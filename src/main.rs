mod util;
mod interface;
mod handler;
mod packet;

#[macro_use]
extern crate log;

use std::collections::HashMap;
use std::env;
use pnet::packet::tcp::TcpFlags;
use util::app::get_arg;
use std::process::exit;
use pnet::datalink;
use pnet::datalink::Channel::Ethernet;
use pnet::packet::ethernet::{EtherTypes, EthernetPacket};

fn main() {
    env::set_var("RUST_LOG", "info");
    env_logger::init();

    let interface_name = get_arg().unwrap();

    let interface = interface::get_from_name(interface_name)
        .unwrap_or_else(|e| {
            error!("{}", e);
            exit(-1);
        });

    let (_tx, mut rx) = match datalink::channel(
        &interface,
        Default::default(),
    ) {
        Ok(Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("Unhandled channel type"),
        Err(_) => {
            error!("Failed to create data link-channel. Try to run with sudo.");
            exit(-1);
        }
    };

    let mut syn_packets = HashMap::new();
    loop {
        let received = match rx.next() {
            Ok(frame) => {
                let frame = EthernetPacket::new(frame).unwrap();
                match frame.get_ethertype() {
                    EtherTypes::Ipv4 => {
                        handler::ip::v4_handler(&frame)
                    },
                    EtherTypes::Ipv6 => {
                        handler::ip::v6_handler(&frame)
                    },
                    _ => {
                        None
                    }
                }
            }
            Err(e) => {
                error!("Failed to read: {}", e);
                None
            }
        };

        if let Some(received) = received {
            debug!("{}", format!("packets: {:?}", syn_packets));
            if ((received.tcp_flags & TcpFlags::SYN) != 0) && ((received.tcp_flags & TcpFlags::ACK) == 0) {
                syn_packets.insert(received.create_key(), received.time);
            }
            else if ((received.tcp_flags & TcpFlags::SYN) == 0) && ((received.tcp_flags & TcpFlags::ACK) != 0) {
                if let Some (&target) = syn_packets.get(&received.create_key()) {
                    info!("{}", format!("[{}] -> [{}], time={:?}", received.l3_src, received.l3_dst, received.time - target));
                    syn_packets.remove(&received.create_key());
                    debug!("{}", format!("packets(after retain): {:?}", syn_packets));
                }
            }
        }
    }
}
