use futures::prelude::*;
use serde_json::json;
use tokio::net::TcpStream;
use tokio_serde::formats::*;
use tokio_util::codec::{FramedWrite, LengthDelimitedCodec, Framed};
use std::thread::sleep;
use futures::executor::block_on;
use std::time::Duration;
use tokio::time;
use log::{info, trace, warn, error};
use simple_logger::SimpleLogger;

#[tokio::main]
pub async fn main() {
    SimpleLogger::new().init().unwrap();
    let mut socket = TcpStream::connect("127.0.0.1:8080").await.unwrap();
    let length_delimited = Framed::new(socket, LengthDelimitedCodec::new());
    let mut serialized =
        tokio_serde::SymmetricallyFramed::new(length_delimited, SymmetricalJson::default());
    let mut interval = time::interval(Duration::from_secs(1));
    loop {

        info!("Send update");
        serialized.send(json!(
        {
            "action": "update"
        }
        )).await.unwrap();

        info!("Receive");
        while let Some(msg) = serialized.try_next().await.unwrap() {
            info!("GOT: {:?} => {}", msg, msg["action"]);
        }
        info!("Receive");
        interval.tick().await;
    }
}