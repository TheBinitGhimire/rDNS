use std::net::{IpAddr};
use dns_lookup::{lookup_addr};

pub fn rDNS(ip_addr: IpAddr) -> String {
	let ip: std::net::IpAddr = format!("{}", ip_addr).parse().unwrap();
    let host = lookup_addr(&ip).unwrap();
    return String::from(host.as_str());
}