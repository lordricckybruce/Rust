Imports

```rust
use pnet::datalink::{self, Channel};
use pnet::packet::{Packet, ethernet::EthernetPacket};
use std::process;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::net::TcpListener;
```

1. `use pnet::datalink::{self, Channel};`:  
   - This line imports the `datalink` module from the `pnet` crate, which provides network interface handling and packet capture.
   - `Channel` is used to handle packet channels for sniffing on network interfaces.
   
2. `use pnet::packet::{Packet, ethernet::EthernetPacket};`:  
   - This imports `Packet` and the `EthernetPacket` structure from the `pnet` crate. `Packet` is a trait for packet handling, and `EthernetPacket` represents Ethernet layer frames.

3. `use std::process;`:  
   - This imports the `process` module, which provides functions to handle the process itself, such as exiting the program with a specific exit status.

4. `use std::net::{IpAddr, Ipv4Addr, SocketAddr};`:  
   - Imports networking types like `IpAddr`, `Ipv4Addr`, and `SocketAddr` from the standard library. These types are useful for managing IP addresses and sockets for network connections.

5. `use std::net::TcpListener;`:  
   - This imports `TcpListener`, which is used for listening on TCP sockets (in this case, for port scanning).

---

### The `main` Function

```rust
fn main() {
    discover_network_interfaces();
    packet_sniffing();
    let target_host = "192.168.1.1"; // Change to your target host
    scan_open_ports(target_host);
}
```

- This is the entry point of the program. It calls three functions:
  1. `discover_network_interfaces()`: Discovers active network interfaces.
  2. `packet_sniffing()`: Starts sniffing network packets.
  3. `scan_open_ports(target_host)`: Scans open ports on a given host (IP address).

---

### 1. Discover Network Interfaces

```rust
fn discover_network_interfaces() {
    let interfaces = datalink::interfaces();
    println!("Active Network Interfaces:");

    for interface in interfaces {
        if interface.is_up() && !interface.is_loopback() {
            println!("{}: {:?}", interface.name, interface.ip_addrs);
        }
    }
}
```

- `datalink::interfaces()`: Fetches all network interfaces available on the machine.
- The loop `for interface in interfaces` iterates over each interface and checks if it's active (`interface.is_up()`) and not a loopback interface (`!interface.is_loopback()`).
- `interface.name` prints the name of the network interface.
- `interface.ip_addrs` prints the IP addresses assigned to the interface.

### What this does:  
It lists the active network interfaces, excluding any loopback interfaces like `localhost`.

---

### 2. Packet Sniffing

```rust
fn packet_sniffing() {
    let interfaces = datalink::interfaces();
    let interface = interfaces
        .iter()
        .find(|iface| iface.is_up() && !iface.is_loopback())
        .expect("No active network interfaces found");

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

                println!("\nCaptured Packet:");
                println!("Source MAC: {:?}", ethernet_packet.get_source());
                println!("Destination MAC: {:?}", ethernet_packet.get_destination());
                println!("Ethernet Type: {:?}", ethernet_packet.get_ethertype());
            }
            Err(_) => eprintln!("Error reading packet"),
        }
    }
}
```

- **Step 1: Select Network Interface**  
   - The program looks for an active, non-loopback network interface using `find`. It selects the first interface that matches the criteria and throws an error if no such interface is found.

- **Step 2: Set Up Packet Capture Channel**  
   - `datalink::channel(&interface, Default::default())` establishes a capture channel (either Ethernet or another protocol) on the selected interface.
   - `Channel::Ethernet(tx, rx)` returns two parts: `tx` (transmit) and `rx` (receive). Here, we are only concerned with the `rx` part for sniffing packets.

- **Step 3: Loop to Capture Packets**  
   - In an infinite loop, it tries to capture packets from the network interface using `rx.next()`.
   - When a packet is received, it is parsed as an Ethernet frame (`EthernetPacket::new(packet)`).
   - The source MAC, destination MAC, and Ethernet type of the captured packet are printed.

### What this does:  
It captures and prints Ethernet frame information (MAC addresses, Ethernet type) from the network traffic.

---

### 3. Scanning Open Ports on a Host

```rust
fn scan_open_ports(host: &str) {
    println!("\nScanning for open ports on host: {}", host);

    let ip_addr: IpAddr = host.parse().expect("Invalid IP address");

    let ipv4_addr = match ip_addr {
        IpAddr::V4(addr) => addr,
        _ => {
            eprintln!("The IP address is not IPv4");
            process::exit(1);
        }
    };

    let ports_to_scan = 1..1024;

    for port in ports_to_scan {
        let socket = SocketAddr::new(ipv4_addr.into(), port);

        match TcpListener::bind(socket) {
            Ok(_) => println!("Port {} is open", port),
            Err(_) => println!("Port {} is closed", port),
        }
    }
}
```

- **Step 1: Parse the Host IP**  
   - The function takes a string `host` and tries to parse it into an `IpAddr` using `host.parse()`. If the host string is invalid, it will terminate the program with an error.

- **Step 2: Convert `IpAddr` to `Ipv4Addr`**  
   - The `match` statement checks if the parsed IP address is an IPv4 address. If it is, it proceeds; if it's not, the program terminates with an error message.

- **Step 3: Scan Ports**  
   - The `ports_to_scan` defines the range of ports to scan (from 1 to 1023).
   - For each port, it tries to bind a `TcpListener` to the IP address and port. If the binding is successful (`Ok(_)`), the port is open; otherwise, it's closed.

### What this does:  
It scans a range of ports (1-1023) on a specified host and reports whether each port is open or closed.

---

### Summary of Functions

- **Network Discovery**: Lists all active network interfaces with their IP addresses.
- **Packet Sniffing**: Captures and decodes Ethernet frames (MAC addresses, Ethernet type) from network traffic.
- **Port Scanning**: Scans a host for open ports (1-1023) using TCP connection attempts.

### Advantages of Rust Over Python for Network Tools:

- **Performance**: Rust is a compiled language, so it typically offers better performance and lower latency compared to Python, especially for high-speed network operations like packet sniffing.
- **Memory Safety**: Rust’s memory safety features (no garbage collection and ownership model) help avoid common bugs such as buffer overflows, which can be more prone to errors in Python.
- **Concurrency**: Rust's ownership and concurrency model allows for efficient multi-threading, which can be useful when handling large amounts of network traffic.

### Disadvantages:

- **Complexity**: Writing and understanding Rust code can be more complex compared to Python due to its stricter typing, ownership system, and compilation step.
- **Longer Development Time**: While Rust's performance is excellent, developing in Rust typically takes longer than Python due to the language's strictness.

### Authentication in Network Tools

The code does not handle authentication or encryption directly. For network scanning and packet sniffing tools, authentication (if required) typically happens at a higher layer (e.g., HTTP or VPN authentication). This code focuses more on network discovery, sniffing, and port scanning rather than authentication.

---


