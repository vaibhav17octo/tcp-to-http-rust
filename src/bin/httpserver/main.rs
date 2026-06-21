use anyhow::anyhow;
use sha2::Digest;
use tcp_to_http::request::Request;
use tcp_to_http::request::headers::Headers;
use tcp_to_http::server;
use tcp_to_http::server::HandlerError;
use tcp_to_http::server::HandlerFuture;
use tcp_to_http::server::response::Response;
use tcp_to_http::server::response::StatusCode;
use tokio::signal;
use sha2::Sha256;

use reqwest::get;

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
</html>"
        .to_vec()
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
</html>"
        .to_vec()
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
</html>"
        .to_vec()
}

fn my_handler(request: Request) -> HandlerFuture {
    Box::pin(async move {
        let mut status = StatusCode::OK;
        let mut body = respond_200();
        let mut headers = Headers::default_response_headers(body.len());
        let mut trailers: Option<Headers> = None;

        if request.request_line.request_target == "/yourproblem" {
            status = StatusCode::BadRequest;
            body = respond_400();
            headers.replace("content-length".to_string(), body.len().to_string());
        } else if request.request_line.request_target == "/myproblem" {
            status = StatusCode::InternalServerError;
            body = respond_500();
            headers.replace("content-length".to_string(), body.len().to_string());
        } else if request.request_line.request_target.starts_with("/httpbin/") {
            let target = request.request_line.request_target["/httpbin/".len()..].to_string();
            match get(format!("https://httpbin.org/{}", target)).await {
                Ok(response) => match response.bytes().await {
                    Ok(b) => {
                        println!("Body from response:{:#?}", b);
                        body = b.to_vec()
                    }
                    Err(e) => {
                        return Err(HandlerError {
                            status: StatusCode::InternalServerError,
                            message: format!("Couldn't convert response to bytes: {}", e)
                                .as_bytes()
                                .to_vec(),
                        });
                    }
                },
                Err(e) => {
                    return Err(HandlerError {
                        status: StatusCode::InternalServerError,
                        message: format!("Couldn't reach out to the URL: {}", e)
                            .as_bytes()
                            .to_vec(),
                    });
                }
            };

            headers.delete("content-length".to_string());
            headers.set("transfer-encoding".to_string(), "chunked".to_string());
            headers.set("trailer".to_string(), "X-Content-SHA256".to_string());
            headers.set("trailer".to_string(), "X-Content-Length".to_string());

            let checksum = hex::encode(Sha256::digest(&body).to_vec());

            trailers = Some(Headers::new());

            if let Some(trailers) = &mut trailers {
              trailers.set("X-Content-SHA256".to_string(), checksum);
              trailers.set("X-Content-Length".to_string(), body.len().to_string());
            }

        }

        Ok(Response::new(status, headers, body, trailers))
    })
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
