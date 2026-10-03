use std::net::TcpStream;
use std::io::stdin;
use std::net::SocketAddr;
use std::time::Duration;
use std::net::IpAddr;

fn main()
{
    let mut input = String::new();
    let mut start_port_input = String::new();
    let mut end_port_input = String::new();
    
    println!("Enter IP address:");
    stdin().read_line(&mut input).expect("Failed to read input.");
    
    println!("Enter starting port:");
    stdin().read_line(&mut start_port_input).expect("Not a port");
    
    println!("Enter ending port:");
    stdin().read_line(&mut end_port_input).expect("Not a port");

    let ip: IpAddr = input.trim().parse().expect("Use an IP address!");

    let start_port: u16 = start_port_input.trim().parse().expect("Use a PORT!");
    let end_port: u16 = end_port_input.trim().parse().expect("Use a PORT!");

    println!("Scanning...");

    let mut counter: usize = 0;

    for port in start_port..=end_port{

        let address = SocketAddr::new(ip, port);

        let result = TcpStream::connect_timeout(&address, Duration::from_secs(1));

    match result {
        Ok(_) => {
            println!("Port {}: CONNECTED", port);
            counter += 1;
        }
        Err(_) => {} //loser does nothing,
    }

    }

    println!("Scan complete.");
    println!("Open ports found: {}", counter);

}