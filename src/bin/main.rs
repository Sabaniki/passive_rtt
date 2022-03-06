#[macro_use]
extern crate log;

use std::collections::HashMap;
use std::env;
use passive_rtt::util::mysql::{establish_connection, RawRtt, update_db};
use passive_rtt::{handler, interface};
use pnet::packet::tcp::TcpFlags;
use passive_rtt::util::app::get_arg;
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

    let connection = establish_connection();


    let mut syn_packets = HashMap::new();
    loop {
        let received = match rx.next() {
            Ok(frame) => {
                let frame = EthernetPacket::new(frame).unwrap();
                match frame.get_ethertype() {
                    // EtherTypes::Ipv4 => {
                    //     handler::ip::v4_handler(&frame)
                    // },
                    EtherTypes::Ipv6 => {
                        let rec = handler::ip::v6_handler(&frame);    
                        // println!("packet tail:{}", "=".repeat(20 * 3));
                        // println!();
                        rec
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
            debug!("{}", format!("syn_packets: {:?}", syn_packets));
            debug!("{}", format!("received_packets: {:?}", received));
            
            if ((received.tcp_flags & TcpFlags::SYN) != 0) && ((received.tcp_flags & TcpFlags::ACK) == 0) {
                syn_packets.insert(received.create_key(), (received.time.clone(), received.sid.clone()));
            }
            else if ((received.tcp_flags & TcpFlags::SYN) == 0) && ((received.tcp_flags & TcpFlags::ACK) != 0) {
                if let Some (target) = syn_packets.get(&received.create_key()) {
                    let rtt =  (received.time - target.0).as_micros() as u32;
                    info!("{}", format!("[{}] -> [{}], time={:?}μs", received.l3_src, received.l3_dst, &rtt));
                    info!("{}", format!("from main, sid: {:?}", received.sid));
                    let sid = target.1.clone();
                    syn_packets.remove(&received.create_key());
                    debug!("{}", format!("packets(after retain): {:?}", syn_packets));
                    let new_rtt = RawRtt {
                        src: received.l3_src,
                        dst: received.l3_dst,
                        sid,
                        rtt
                    };
                    update_db(&new_rtt, &connection);
                }
            }
            else if((received.tcp_flags & TcpFlags::SYN) != 0) && ((received.tcp_flags & TcpFlags::ACK) != 0)  {
                debug!("{}", format!("on syn-ack"));
                if let Some (target) = syn_packets.get(&received.reverse_create_key()) {
                    debug!("{}", format!("hit key!"));
                    syn_packets.insert(received.reverse_create_key(), (target.0, received.sid));
                }
            }
            // println!("end received:{}", "=".repeat(20 * 3));
        }
    }
}
