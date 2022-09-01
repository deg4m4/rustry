mod pty;
mod raw_guard;

use filedescriptor::AsRawSocketDescriptor;
use futures::{sink::SinkExt, stream::StreamExt};
use hyper_tungstenite::{tungstenite, HyperWebsocket};
use libc::printf;
use std::io::prelude::*;
use std::rc::Rc;
use std::sync::Arc;
use std::{convert::Infallible, fs::File};
use tokio::sync::Mutex;
use tungstenite::Message;

use pty::run_term;
use pty_process::Command as _;
use std::os::unix::process::ExitStatusExt as _;

pub async fn serve_websocket(websocket: HyperWebsocket) -> Result<(), Infallible> {
    let websocket = websocket.await.unwrap();
    let ws = Arc::new(Mutex::new(websocket));
    //let p = Arc::new(Mutex::new(&websocket));
    // let (tx, rx) = mpsc::channel();
    // let mut file = File::create("/tmp/parthka").unwrap();
    // let mut file2 = file.try_clone().unwrap();

    let (mut a, mut b) = filedescriptor::socketpair().unwrap();

    tokio::spawn({
        let ws = ws.clone();
        async move {
            
            run_term(&mut a, ws.clone()).await;
        }
    });

    /*     std::thread::spawn({
        move || { /* websocket.send(Message::Text("asd".into())); */ }
    }); */

    while let Some(message) = ws.lock().await.next().await {
        if let Message::Text(msg) = message.unwrap() {
            b.write(msg.as_bytes()).unwrap();
        }
    }
    println!("Exit123");

    Ok(())
}
