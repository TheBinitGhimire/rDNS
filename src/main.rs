#![allow(non_snake_case)]
mod rDNS;

use std::{process::exit};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::net::{IpAddr};

use clap::{Arg, App};

fn main() -> std::io::Result<()> {
	let args = App::new("rDNS")
			.version("0.0.1")
			.author("Binit Ghimire <binit@WHOISbinit.me>")
			.about("rDNS: Reverse DNS Lookup Utility!")
			.args(&[
				Arg::new("file")
					.about("List of IP Addresses separated with new line (\\n)!")
					.short('f')
					.long("file")
					.takes_value(true),
				Arg::new("target")
					.about("An IP Address!")
					.short('t')
					.long("target")
					.takes_value(true),
			]).get_matches();
	if args.is_present("file") && args.is_present("target") {
		println!("Please provide either a single IP address or a file containing list of IPs rather than both!");
		exit(1);
	} else if args.is_present("file"){
		let ipaddrs = args.value_of("file").unwrap_or("list.txt");
		let file = File::open(ipaddrs).unwrap();
		let reader = BufReader::new(file);
		let mut IPs = Vec::<String>::new();
		for line in reader.lines(){
			IPs.push(line.unwrap());
		}
		lookup(IPs);
	} else if args.is_present("target"){
		let _target = args.value_of("target").unwrap();
		let mut IPs = Vec::<String>::new();
		IPs.push(_target.to_string());
		lookup(IPs);
	}
	Ok(())
}

fn lookup(IPs: Vec<String>){
	for input in IPs {
		println!("Input: {}", input);
		let addr = input.parse::<IpAddr>().unwrap();
		println!("Hostname: {}\n", rDNS::rDNS(addr));
	}
}

