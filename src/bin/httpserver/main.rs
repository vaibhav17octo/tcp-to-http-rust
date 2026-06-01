use anyhow::anyhow;
use tcp_to_http::server;
use tokio::signal;
use tcp_to_http::request::Request;
use std::io::Write;
use tcp_to_http::server::response::StatusCode;
use tcp_to_http::server::HandlerError;

const PORT: u16 = 42069;

async fn shutdown_signal() {
    signal::ctrl_c().await.expect("Failed to listen for ctrl+c");
}

fn my_handler(write: &mut dyn Write, request: Request) -> Result<(), HandlerError> {
    if request.request_line.request_target == "/yourproblem" {
        return Err(HandlerError {
            status: StatusCode::BadRequest,
            message: b"Your problem is not my problem\n".to_vec()
        })
    }
    else if request.request_line.request_target == "/myproblem" {
        return Err(HandlerError {
            status: StatusCode::InternalServerError,
            message: b"Woopsie, my bad\n".to_vec()
        })
    }
    else {
        match write.write(b"All good, frfr\n") {
            Ok(_u) => {},
            Err(_e) => {
                return Err(HandlerError {
                    status: StatusCode::InternalServerError,
                    message: b"Could not write to stream\n".to_vec()
                })
            }
        };
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    tokio::select! {
        res = server::serve(PORT, my_handler) => {
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
