use anyhow;
use tcp_to_http::request::request_from_reader;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let ln = TcpListener::bind("127.0.0.1:42069").await?;

    loop {
        let (mut stream, _addr) = ln.accept().await?;

        let request = request_from_reader(&mut stream).await?;
        println!("{}", &request);
    }
}
