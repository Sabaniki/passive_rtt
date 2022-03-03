#[macro_use]
extern crate log;

use std::collections::HashMap;
use std::env;
use passive_rtt::util::mysql::{establish_connection, Rtt};
use passive_rtt::{handler, interface};
use pnet::packet::tcp::TcpFlags;
use passive_rtt::util::app::get_arg;
use std::process::exit;
use pnet::datalink;
use pnet::datalink::Channel::Ethernet;
use pnet::packet::ethernet::{EtherTypes, EthernetPacket};
use diesel::insert_into;
use diesel::RunQueryDsl;
use crypto::sha2::Sha256;
use crypto::digest::Digest;

fn main() {
    env::set_var("RUST_LOG", "debug");
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

    let connection = establish_connection();

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
                    let rtt =  (received.time - target).as_micros() as u32;
                    info!("{}", format!("[{}] -> [{}], time={:?}μs", received.l3_src, received.l3_dst, &rtt));
                    syn_packets.remove(&received.create_key());
                    debug!("{}", format!("packets(after retain): {:?}", syn_packets));
                    let hash = received.l3_src.clone() + &received.l3_dst.clone();
            
                    let mut sha256 = Sha256::new();
                    sha256.input_str(&hash);
                    let new_rtt = Rtt {
                        id: sha256.result_str(),
                        src: received.l3_src,
                        dst: received.l3_dst,
                        rtt: rtt
                    };
                    insert_into(passive_rtt::schema::rtts::dsl::rtts)
                        .values(new_rtt)
                        .execute(&connection)
                        .expect("Error saving new rtt");
                    debug!("{}", format!("done!!!"));
                }
            }
        }
    }
}
