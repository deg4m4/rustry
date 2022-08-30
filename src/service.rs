use hyper::{Body, Request, Response};
use std::{convert::Infallible, time::{self, UNIX_EPOCH}};
use futures::{sink::SinkExt, stream::StreamExt};
use hyper_tungstenite::{tungstenite, HyperWebsocket};
use tungstenite::Message;

pub async fn main_handler(mut req: Request<Body>) -> Result<Response<Body>, Infallible> {
    if req.uri() != "/so" {
        return Ok(Response::new("Hello, World".into()))
    } 

    if hyper_tungstenite::is_upgrade_request(&req) {
        let (response, websocket) = hyper_tungstenite::upgrade(&mut req, None).unwrap();

        // Spawn a task to handle the websocket connection.
        tokio::spawn(async move {
            if let Err(e) = serve_websocket(websocket).await {
                eprintln!("Error in websocket connection: {}", e);
            }
        });

        // Return the response so the spawned future can continue.
        Ok(response)
    } else {
        // Handle regular HTTP requests here.
        Ok(Response::new(Body::from("Hello HTTP!")))
    }

}

/// Handle a websocket connection.
async fn serve_websocket(websocket: HyperWebsocket) -> Result<(), Infallible> {
    let time = time::SystemTime::now().duration_since(UNIX_EPOCH);
    let mut websocket = websocket.await.unwrap();
    while let Some(message) = websocket.next().await {
        match message.unwrap() {
            Message::Text(msg) => {
                println!("Received text message: {}", msg);
                if msg == "Hello, Parth!".to_string() {
                    websocket.send(Message::text("Hello, King!")).await;
                } else {
                    websocket.send(Message::text(format!("Thank you, {:?}.", time))).await;
                }
            },
            Message::Binary(msg) => {
                println!("Received binary message: {:02X?}", msg);
                websocket.send(Message::binary(b"Thank you, come again.".to_vec())).await;
            },
            Message::Ping(msg) => {
                // No need to send a reply: tungstenite takes care of this for you.
                println!("Received ping message: {:02X?}", msg);
            },
            Message::Pong(msg) => {
                println!("Received pong message: {:02X?}", msg);
            }
            Message::Close(msg) => {
                // No need to send a reply: tungstenite takes care of this for you.
                if let Some(msg) = &msg {
                    println!("Received close message with code {} and message: {}", msg.code, msg.reason);
                } else {
                    println!("Received close message");
                }
            },
            Message::Frame(msg) => {
               unreachable!();
            }
        }
    }

    Ok(())
}

