use anyhow::anyhow;
use tcp_to_http::server;
use tokio::signal;

const PORT: u16 = 42069;

async fn shutdown_signal() {
    signal::ctrl_c().await.expect("Failed to listen for ctrl+c");
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    tokio::select! {
        res = server::serve(PORT) => {
            match res {
                Ok(_s) => println!("Server will shutdown gracefully"),
                Err(e) => return Err(anyhow!(format!("{}", e)))
            }
        }
        _ = shutdown_signal() => {
            println!("Sig chan received, shutting down");
        }
    }
    Ok(())
}
