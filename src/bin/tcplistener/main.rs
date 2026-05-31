use anyhow;
use tokio::net::TcpListener;
use tcp_to_http::request::request_from_reader;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let ln = TcpListener::bind("127.0.0.1:42069").await?;

    loop {
        let (stream, _addr) = ln.accept().await?;
        
        let request = request_from_reader(stream).await?;
        println!("{}", &request);
    }
}
