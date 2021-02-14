use std::io::prelude::*;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpStream, Shutdown};
use std::time::Duration;
use std::thread;

fn main() -> std::io::Result<()> {
    let mut i = 9090;
    let timeout = Duration::from_millis(50);
    println!("starting port: {}", i);

    loop {
        println!(
            "Try connect to port: {} with timeout {} ms",
            i,
            timeout.as_millis()
        );
        let socket_addr = &SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), i);
        let box_stream = TcpStream::connect_timeout(socket_addr, timeout);

        let mut stream = match box_stream {
            Ok(stream) => stream,
            Err(_) => {
                i = i + 1;
                continue;
            }
        };

        println!("Connected to {}, try to send byte", i);
        stream.set_read_timeout(Some(Duration::from_secs(1)))?;

        match stream.write(&[1]) {
            Ok(_) => {
                println!("Data sended");
            }
            Err(err) => {
                println!("Data can't send {}", err);
                continue;
            }
        }
        let mut buf = [];
        match stream.read(&mut [0; 128]) {
            Ok(data) => {
                println!("Data recieved {}", data);
            },
            Err(_err) => {
                println!("Data reciev error {}", _err);
            }
        };
        thread::sleep(Duration::from_millis(10));

        match stream.shutdown(Shutdown::Both) {
            Ok(_) => {
                println!("Shutdown success");
            },
            Err(_) => {
                println!("Shutdown error");
            }
        };

        break;
    }

    Ok(())
}
