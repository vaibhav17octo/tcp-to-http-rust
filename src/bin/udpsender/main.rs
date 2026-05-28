use std::io;
use std::net::UdpSocket;
fn main() {
    let sock = UdpSocket::bind("0.0.0.0:42068").unwrap();

    sock.connect("127.0.0.1:42069").unwrap();
    loop {
        let mut str = String::new();
        io::stdin()
            .read_line(&mut str)
            .expect("Did not enter string");

        match sock.send(str.as_bytes()) {
            Ok(len) => println!("Send {} bytes", len),
            Err(e) => println!("Error occured: {}", e),
        }
    }
}
