mod util;
mod interface;
mod handler;
mod packet;

#[macro_use]
extern crate log;

use std::env;
use packet::tuples::FiveTupleWithFlagsAndTime;
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

    // 引数からインターフェイスを選択
    let interface = interface::get_from_name(interface_name)
        .unwrap_or_else(|e| {
            error!("{}", e);
            exit(-1);
        });

    // データリンクのチャンネルを取得
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

    // let mut before: Option<FiveTupleWithFlagsAndTime> = None;
    // let mut waiting_ack = false;
    let mut syn_packets = Vec::new();
    loop {
        let received = match rx.next() {
            Ok(frame) => {
                // 受信パケットからイーサネットフレームを構築
                let frame = EthernetPacket::new(frame).unwrap();
                match frame.get_ethertype() {
                    EtherTypes::Ipv4 => {
                        handler::ip::v4_handler(&frame)
                    },
                    EtherTypes::Ipv6 => {
                        handler::ip::v6_handler(&frame)
                    },
                    _ => {
                        // info!("This packet is neither IPv4 nor IPv6");
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
            let same_src_and_dst = syn_packets.iter()
            .filter(|&packet| 
                FiveTupleWithFlagsAndTime::is_same_src_and_dst(&received, packet)
            )
            .collect::<Vec<&FiveTupleWithFlagsAndTime>>();

            if ((received.tcp_flags & TcpFlags::SYN) != 0) && ((received.tcp_flags & TcpFlags::ACK) == 0) {
                let count = same_src_and_dst.len();
                if count == 0 {
                    syn_packets.push(received);
                }
            }
            else if ((received.tcp_flags & TcpFlags::SYN) == 0) && ((received.tcp_flags & TcpFlags::ACK) != 0) {
                let target = same_src_and_dst.get(0);
                if let Some (target) = target {
                    info!("{}", format!("[{}] -> [{}], time={:?}", received.l3_src, target.l3_dst, received.time - target.time));
                    syn_packets.retain(|packet| 
                        !FiveTupleWithFlagsAndTime::is_same_src_and_dst(&received, packet)
                    );
                    debug!("{}", format!("packets(after retain): {:?}", syn_packets));
                }
            }
        }
    }
}
