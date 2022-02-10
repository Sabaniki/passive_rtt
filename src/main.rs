mod util;
mod interface;
mod handler;
mod packet;

#[macro_use]
extern crate log;

use std::env;
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

    loop {
        match rx.next() {
            Ok(frame) => {
                // 受信パケットからイーサネットフレームを構築
                let frame = EthernetPacket::new(frame).unwrap();
                match frame.get_ethertype() {
                    EtherTypes::Ipv4 => {
                        handler::ip::v4_handler(&frame);
                    },
                    EtherTypes::Ipv6 => {
                        handler::ip::v6_handler(&frame);
                    },
                    _ => {
                        info!("This packet is neither IPv4 nor IPv6");
                    }
                }
            }
            Err(e) => {
                error!("Failed to read: {}", e);
            }
        }
    }
}
