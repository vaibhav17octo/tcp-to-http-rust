use anyhow::anyhow;
use tcp_to_http::server;
use tcp_to_http::server::response::Response;
use tokio::signal;
use tcp_to_http::request::Request;
use tcp_to_http::server::response::StatusCode;
use tcp_to_http::server::HandlerError;
use tcp_to_http::request::headers::Headers;

const PORT: u16 = 42069;

async fn shutdown_signal() {
    signal::ctrl_c().await.expect("Failed to listen for ctrl+c");
}

fn respond_400() -> Vec<u8> {
    b"<!DOCTYPE html>
  <head>
    <title>400 Bad Request</title>
  </head>
  <body>
    <h1>Bad Request</h1>
    <p>Your request honestly kinda sucked.</p>
  </body>
</html>".to_vec()
}

fn respond_500() -> Vec<u8> {
    b"<!DOCTYPE html>
  <head>
    <title>500 Internal Server Error</title>
  </head>
  <body>
    <h1>Internal Server Error</h1>
    <p>Okay, you know what? This one is on me.</p>
  </body>
</html>".to_vec()
}

fn respond_200() -> Vec<u8> {
    b"<!DOCTYPE html>
  <head>
    <title>200 OK</title>
  </head>
  <body>
    <h1>Success!</h1>
    <p>Your request was an absolute banger.</p>
  </body>
</html>".to_vec()
}

fn my_handler(request: Request) -> Result<Response, HandlerError> {

    let mut status = StatusCode::OK;
    let mut body = respond_200();

    if request.request_line.request_target == "/yourproblem" {
        status = StatusCode::BadRequest;
        body = respond_400();
    }
    else if request.request_line.request_target == "/myproblem" {
        status = StatusCode::InternalServerError;
        body = respond_500();
    }
    
    let headers = Headers::default_response_headers(body.len());

    Ok(Response::new(status, headers, body))
    
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
