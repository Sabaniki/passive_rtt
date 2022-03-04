use pnet::packet::ip::IpNextHeaderProtocols;
use pnet::packet::ipv4::Ipv4Packet;
use pnet::packet::ipv6::Ipv6Packet;
use pnet::packet::Packet;

// pub trait L3Packet {
//     fn get_source(&self) -> String;
//     fn get_destination(&self) -> String;
//     fn get_payload(&self) -> &[u8];
//     fn get_sid(&self) -> Option<String>;
// }


// impl<'a> L3Packet for Ipv4Packet<'a> {
//     fn get_source(&self) -> String {
//         self.get_source().to_string()
//     }

//     fn get_destination(&self) -> String {
//         self.get_destination().to_string()
//     }

//     fn get_payload(&self) -> &[u8] {
//         self.payload()
//     }

//     fn get_sid(&self) -> Option<String> {
//         None
//     }
// }

// impl<'a> L3Packet for Ipv6Packet<'a> {
//     fn get_source(&self) -> String {
//         self.get_source().to_string()
//     }

//     fn get_destination(&self) -> String {
//         self.get_destination().to_string()
//     }

//     fn get_payload(&self) -> &[u8] {
//         self.payload()
//     }

//     fn get_sid(&self) -> Option<String> {
//         let header = self.get_next_header();
//         let sid = match header {
//             IpNextHeaderProtocols::Ipv6Route => {
//                 Some(self.get_destination().to_string())
//             },
//             _ => {
//                 None
//             }
//         };
//         sid
//     }
// }