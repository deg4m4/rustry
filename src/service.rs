use super::pty::serve_websocket;
use hyper::{Body, Request, Response};
use std::convert::Infallible;
use tokio::{fs::File, io::AsyncReadExt};

pub async fn main_handler(mut req: Request<Body>) -> Result<Response<Body>, Infallible> {
    if req.uri() != "/so" {
        let mut p = File::open("term.html").await.unwrap();
        let mut buf = Vec::new();
        p.read_to_end(&mut buf).await.unwrap();
        return Ok(Response::new(buf.into()));
    }

    if hyper_tungstenite::is_upgrade_request(&req) {
        let (response, websocket) = hyper_tungstenite::upgrade(&mut req, None).unwrap();

        tokio::spawn(async move {
            if let Err(e) = serve_websocket(websocket).await {
                eprintln!("Error in websocket connection: {}", e);
            }
        });

        Ok(response)
    } else {
        Ok(Response::new(Body::from("Hello HTTP!")))
    }
}
