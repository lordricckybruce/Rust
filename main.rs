use pnet::datalink::{self, Channel}; // to import datalink module from pnet crate
use pnet::packet::{Packet, ethernet::EthernetPacket}; //for packet handling, imports packet  and ethernetpacket structure from pnet crate
use std::process; //imports process module
use std::net::{IpAddr, Ipv4Addr, SocketAddr}; //import networking module for managing  ip addresses
use std::net::TcpListener; // used for listening on tcp sockets, port scanning

fn main() {
    // Function 1: Discover Active Network Interfaces
    discover_network_interfaces();

    // Function 2: Start Packet Sniffing
    packet_sniffing();

    // Function 3: Scan for Open Ports on a Specific Host (Optional)
    let target_host = "localhost"; // Change to your target host
    scan_open_ports(target_host);
}

// Function 1: Discover Active Network Interfaces
fn discover_network_interfaces() {
    let interfaces = datalink::interfaces(); //fetches all networks interface availabale on machine
    println!("Active Network Interfaces:");

    for interface in interfaces {  //iterates over each interface and checks for active knterface.is_up and loop back
        if interface.is_up() && !interface.is_loopback() {
            println!("{}: {:?}", interface.name, interface.ip_addrs);//interface name prints name of machine, interface.ip_addrs prints IP addresses assigned to the interface
        }
    }
}

// Function 2: Sniff Network Packets
fn packet_sniffing() {
    // Get network interfaces
    let interfaces = datalink::interfaces();
    let interface = interfaces
        .iter()
        .find(|iface| iface.is_up() && !iface.is_loopback())
        .expect("No active network interfaces found");

    // Set up the capture channel for the chosen interface
    let (mut tx, mut rx) = match datalink::channel(&interface, Default::default()) {
        Ok(Channel::Ethernet(tx, rx)) => (tx, rx),
        _ => {
            eprintln!("Error setting up packet capture");
            process::exit(1);
        }
    };

    println!("\nListening on interface: {}", interface.name);

    loop {
        match rx.next() {
            Ok(packet) => {
                let ethernet_packet = EthernetPacket::new(packet).expect("Failed to parse Ethernet packet");

                // Print basic Ethernet packet information
                println!("\nCaptured Packet:");
                println!("Source MAC: {:?}", ethernet_packet.get_source());
                println!("Destination MAC: {:?}", ethernet_packet.get_destination());
                println!("Ethernet Type: {:?}", ethernet_packet.get_ethertype());
            }
            Err(_) => eprintln!("Error reading packet"),
        }
    }
}

// Function 3: Scan for Open Ports on a Specific Host
fn scan_open_ports(host: &str) {
    println!("\nScanning for open ports on host: {}", host);

    let ip_addr: IpAddr = host.parse().expect("Invalid IP address");
    
    // Convert IpAddr to Ipv4Addr safely
    let ipv4_addr = match ip_addr {
        IpAddr::V4(addr) => addr,
        _ => {
            eprintln!("The IP address is not IPv4");
            process::exit(1);
        }
    };

    // Set the range of ports to scan
    let ports_to_scan = 1..1024; // Change port range as needed

    for port in ports_to_scan {
        let socket = SocketAddr::new(ipv4_addr.into(), port);

        match TcpListener::bind(socket) {
            Ok(_) => println!("Port {} is open", port),
            Err(_) => println!("Port {} is closed", port),
        }
    }
}

