use std::net::TcpStream;
use std::io::stdin;
use std::net::SocketAddr;
use std::time::Duration;
use std::net::IpAddr;

fn main()
{
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Failed to read input.");
    println!("{}", input);

    let ip: IpAddr = input.trim().parse().expect("Use an IP address!");
    
    for port in 7900..8001{

        let address = SocketAddr::new(ip, port);

        let result = TcpStream::connect_timeout(&address, Duration::from_secs(1));

    match result {
        Ok(_) => println!("Status: CONNECTED."),
        Err(_) => println!("Status: CLOSED"),
    }

    }

}