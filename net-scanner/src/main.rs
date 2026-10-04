use std::net::{TcpStream, SocketAddr, IpAddr};
use std::io::stdin;
use std::time::Duration;
use std::thread;

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
    loop {
        let mut input = String::new();
        println!("Enter IP address:");
        stdin().read_line(&mut input).expect("Failed to read input.");

        match input.trim().parse() {
            Ok(ip) => return ip,
            Err(_) => println!("Invalid IP!"),
        }
    }
}

fn get_port(prompt: &str) -> u16 {
    loop{ 
        let mut port_input = String::new();
        println!("{}", prompt);
        stdin().read_line(&mut port_input).expect("Use a PORT.");

        match port_input.trim().parse() {
            Ok(port) => {
                return port;
            }
            Err(_) => println!("Invalid port!"),
        }
    }
    
}

fn scan_ports(ip_address: IpAddr, start_port: u16, end_port: u16) -> usize {
    let mut counter:usize = 0;
    let mut vector = Vec::new();
    
    for port in start_port..=end_port {
    let address = SocketAddr::new(ip_address, port); 
    let multiple_connections = thread::spawn(move || {
        let connection_result = TcpStream::connect_timeout(&address, Duration::from_secs(1));
    
        (port, connection_result)
    });
    vector.push(multiple_connections);
    }

    for handle in vector {
        let result = handle.join();
    
        match result {
            Ok((port, connection_result)) => {
                match connection_result {
                    Ok(_) => {
                        println!("Port {}: CONNECTED", port); 
                        counter += 1;
                    }

                    Err(_) => {

                    }
                }
            }
            Err(_) => {
                //thread failed
            }
        
        }
    
    }
    counter
}