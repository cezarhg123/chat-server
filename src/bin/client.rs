use std::{io::{Write, Read}, net::{ToSocketAddrs}, thread};

fn main () -> ! {
    let mut address: String = String::new();
    
    print!("Ip and port to connect to : ");
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut address).unwrap();

    let remote_address = address.trim().to_socket_addrs().unwrap().as_slice()[0];
    let mut socket = std::net::TcpStream::connect(&remote_address).unwrap();
    let mut socket_clone = socket.try_clone().unwrap();

    thread::spawn(move || {
        loop {
            let mut buffer: [u8; 1000] = [0; 1000];
            let _ = socket_clone.read(&mut buffer).unwrap();
            let string = String::from_utf8_lossy(&buffer).to_string();
            println!("{}", string);
        }
    });

    loop {
        let mut input = String::new();

        std::io::stdin().read_line(&mut input).unwrap();
        if input.len() >= 1000 {
            println!("send less than 1000 charactes shithead");
        }

        else {
            let _ = socket.write(input.as_bytes()).unwrap();
        }
    }
}