use std::net::TcpStream;
use std::io::stdin;
use std::net::SocketAddr;
use std::time::Duration;

fn main()
{
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Failed to read input.");
    println!("{}", input);

    let ip: SocketAddr = input.trim().parse().expect("Use an IP address!");
    let result = TcpStream::connect_timeout(&ip, Duration::from_secs(1));
    println!("{:?}", result)
}