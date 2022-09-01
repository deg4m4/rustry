use filedescriptor::{
    poll, pollfd, AsRawFileDescriptor, AsRawSocketDescriptor, FileDescriptor, POLLIN,
};
use futures::{SinkExt, StreamExt};
use hyper::upgrade::Upgraded;
use hyper_tungstenite::tungstenite::Message;
use hyper_tungstenite::WebSocketStream;
use std::fs::File;
use std::io::{Read as _, Write as _};
use std::os::unix::io::AsRawFd as _;
use std::rc::Rc;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use pty_process::Command as _;
use std::os::unix::process::ExitStatusExt as _;

pub async fn run_term(
    rx: &mut FileDescriptor,
    ws: Arc<Mutex<WebSocketStream<Upgraded>>>,
) {
    let child = std::process::Command::new("bash")
                .spawn_pty(Some(&pty_process::Size::new(24, 80)))
                .unwrap();
    let _raw = super::raw_guard::RawGuard::new();
    let mut buf = [0_u8; 4096];

    //let pty = child.pty().as_raw_fd();
    let mut poll_array = [pollfd {
        fd: rx.as_socket_descriptor(),
        events: POLLIN,
        revents: 0,
    }];

    let mut poll_array_2 = [pollfd {
        fd: child.pty().as_raw_file_descriptor(),
        events: POLLIN,
        revents: 0,
    }];

    loop {
        if poll(&mut poll_array, None).unwrap() > 0 {
            match rx.read(&mut buf) {
                Ok(bytes) => {
                    let buf = &buf[..bytes];
                    child.pty().write_all(buf).unwrap();
                }
                Err(e) => {
                    eprintln!("stdin read failed: {:?}", e);
                    break;
                }
            }
        }

        if poll(&mut poll_array_2, None).unwrap() > 0 {
            println!("hello");
            match child.pty().read(&mut buf) {
                Ok(bytes) => {
                    let buf = &buf[..bytes];
                    ws.lock()
                        .await
                        .send(Message::text(std::str::from_utf8(buf).unwrap()))
                        .await
                        .unwrap();
                }
                Err(e) => {
                    if e.raw_os_error() != Some(libc::EIO) {
                        eprintln!("pty read failed: {:?}", e);
                    }
                    break;
                }
            };
        }
    }
}
