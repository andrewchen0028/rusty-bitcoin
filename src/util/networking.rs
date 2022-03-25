use std::net::{Ipv4Addr, SocketAddrV4};

use super::constants::RBTC_PORT;

pub fn addr_from_node_number(node_number: u8) -> SocketAddrV4 {
  SocketAddrV4::new(
    Ipv4Addr::new(
      Ipv4Addr::LOCALHOST.octets()[0],
      Ipv4Addr::LOCALHOST.octets()[1],
      Ipv4Addr::LOCALHOST.octets()[2],
      node_number,
    ),
    RBTC_PORT,
  )
}
