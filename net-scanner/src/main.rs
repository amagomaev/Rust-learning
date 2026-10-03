use std::net::TcpStream;
use std::io::stdin;
use std::net::SocketAddr;
use std::time::Duration;
use std::net::IpAddr;

fn main() {
    println!("Scanning...");
    let ip = get_ip();
    let start_port = get_port("Enter starting port:");
    let end_port = get_port("Enter ending port:");
    let counter = scan_ports(ip, start_port, end_port);

    
    println!("Scan complete.");
    println!("Open ports found: {}", counter);
}

fn get_ip() -> IpAddr {
    let mut input = String::new();
    println!("Enter IP address:");
    stdin().read_line(&mut input).expect("Failed to read input.");
    let ip = input.trim().parse().expect("Use an IP address!");
    ip
}

fn get_port(prompt: &str) -> u16 {
    let mut port_input = String::new();
    println!("{}", prompt);
    stdin().read_line(&mut port_input).expect("Use a PORT.");
    port_input.trim().parse().expect("Use a PORT!")
}

fn scan_ports(ip_address: IpAddr, start_port: u16, end_port: u16) -> usize {
    let mut counter:usize = 0;
    
    for port in start_port..=end_port {
    let address = SocketAddr::new(ip_address, port); 
    let result = TcpStream::connect_timeout(&address, Duration::from_secs(1));
    match result {
        Ok(_) => {
            println!("Port {}: CONNECTED", port);
            counter += 1;
        }
        Err(_) => {}
        }
    }
    counter
}