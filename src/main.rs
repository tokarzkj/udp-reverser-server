use std::net::UdpSocket;
use std::str;

#[macro_use]
extern crate log;
extern crate simple_logger;

fn main() -> std::io::Result<()> {
    {
        simple_logger::init().unwrap();

        let socket = UdpSocket::bind("127.0.0.1:34254")?;
        loop {
            let mut buf = [0; 50];
            let (amt, src) = socket.recv_from(&mut buf)?;

            let buf = &mut buf[..amt];
            let message = str::from_utf8(&buf).expect("decoded string");
            if message == "quit" {
                break;
            }

            info!("The original message is {}", str::from_utf8(&buf).unwrap());
            buf.reverse();            
            info!("The reversed message is {}", str::from_utf8(&buf).unwrap());
            socket.send_to(buf, &src)?;
        }
    }
    Ok(())
}
