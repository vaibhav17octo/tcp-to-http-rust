use anyhow;
use std::net::TcpListener;
use tcp_to_http::request::request_from_reader;
fn main() -> Result<(), anyhow::Error> {
    let ln = TcpListener::bind("127.0.0.1:42069")?;

    for stream in ln.incoming() {
        match stream {
            Ok(s) => {
                let request = request_from_reader(s)?;
                
                println!("{}", &request);
            },
            Err(e) => println!("{}", e),
        }
    }
   
    Ok(())
}
