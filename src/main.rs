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
    env::set_var("RUST_LOG", "debug");
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

    let mut before: Option<FiveTupleWithFlagsAndTime> = None;
    let mut waiting_ack = false;
    loop {
        let tmp = match rx.next() {
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


        if let Some (tmp) = tmp {
            if waiting_ack {
                if let Some(ref before) = before {
                    if FiveTupleWithFlagsAndTime::is_same_src_and_dst(&tmp, before) && (tmp.tcp_flags & TcpFlags::ACK) != 0{
                        info!("{}", format!("[{}] -> [{}], time={:?}", tmp.l3_src, tmp.l3_dst, tmp.time - before.time));
                        waiting_ack = false;
                    }
                }
            }
            if !waiting_ack && (tmp.tcp_flags & TcpFlags::SYN) != 0 {
                before = Some(tmp);
                waiting_ack = true;
                // info!("SYN を検知");
            }
        }
    }
}
