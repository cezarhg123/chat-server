use std::{io::{Read, Write}, thread};

fn main() {
    let listerner = std::net::TcpListener::bind("192.168.1.12:69").unwrap();
    //                                          ^^^^^^^^^^^^^ - change this ip to the host's local ip 
    let (mut stream1, _remote_address1) = listerner.accept().unwrap();
    let (mut stream2, _remote_address2) = listerner.accept().unwrap();
    let mut stream1_clone = stream1.try_clone().unwrap();
    let mut stream2_clone = stream2.try_clone().unwrap();

    thread::spawn(move || {
        loop {
            let mut buffer1:[u8; 1000] = [0; 1000];
            let received_bytes = stream1.read(&mut buffer1).unwrap();
            println!("received {} bytes", received_bytes);
            let received_bytes = stream2.write(&buffer1).unwrap();
            println!("sent {} bytes", received_bytes);
        }
    });

    loop {
        let mut buffer2:[u8; 1000] = [0; 1000];
        let received_bytes = stream2_clone.read(&mut buffer2).unwrap();
        println!("received {} bytes", received_bytes);
        let received_bytes = stream1_clone.write(&buffer2).unwrap();
        println!("sent {} bytes", received_bytes);
    }
}
