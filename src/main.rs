use std::net::UdpSocket;
use std::str;

fn main() -> std::io::Result<()> {
    {
        let socket = UdpSocket::bind("127.0.0.1:34254")?;
        loop {
            let mut buf = [0; 50];
            let (amt, src) = socket.recv_from(&mut buf)?;

            let buf = &mut buf[..amt];
            let message = str::from_utf8(&buf).expect("decoded string");
            if message == "quit" {
                break;
            }

            buf.reverse();
            socket.send_to(buf, &src)?;
        }
    }
    Ok(())
}
