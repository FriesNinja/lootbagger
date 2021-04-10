extern crate daemonize;

use log::{info, trace, warn, error};
use clap::{App, Arg};

use serde_json::json;
use std::fs::File;

use daemonize::Daemonize;
use simple_logger::SimpleLogger;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio::runtime::Runtime;
use std::error::Error;

use futures::prelude::*;
use serde_json::Value;
use tokio_serde::formats::*;
use tokio_util::codec::{FramedRead, LengthDelimitedCodec, Framed};

fn start_server() ->  Result<(), Box<dyn Error>> {
    info!("Success, daemonized");
    let mut rt = Runtime::new()?;

    rt.block_on(async {
        let mut listener = TcpListener::bind("127.0.0.1:8080").await?;

        loop {
            let (socket, _) = listener.accept().await.unwrap();
            let length_delimited = Framed::new(socket, LengthDelimitedCodec::new());

            let mut deserialized = tokio_serde::SymmetricallyFramed::new(
                length_delimited,
                SymmetricalJson::<Value>::default(),
            );

            tokio::spawn(async move {
                while let Some(msg) = deserialized.try_next().await.unwrap() {
                    info!("GOT: {:?} => {}", msg, msg["action"]);
                }

                let response = json!(
                        {
                            "action": "test"
                        }
                );

                info!("Response message {} -> {}", response.to_string(), response.to_string().len());
                deserialized.send(response).await.unwrap();
                info!("Next Ply");
            });
        }
    })
}

fn daemonize() {
    let stdout = File::create("/tmp/daemon.out").unwrap();
    let stderr = File::create("/tmp/daemon.err").unwrap();

    let daemonize = Daemonize::new()
        .pid_file("/tmp/test.pid")
        .working_directory("/tmp")
        .stdout(stdout)
        .stderr(stderr);

    match daemonize.start() {
        Ok(_) => start_server().unwrap(),
        Err(e) => error!("Error, {}", e),
    }
}


fn main() {
    SimpleLogger::new().init().unwrap();

    let matches = App::new("LootBagger Daemon")
        .version("0.1")
        .author("Christian Fries <christian.fries123@gmail.com")
        .about("Daemon to gather loot for different projects")
        .arg(Arg::new("foreground")
            .about("Start in foreground")
            .short('f'))
        .get_matches();


    if matches.is_present("foreground") {
        start_server().expect("Could not start a new server process");
    } else {
        /* Start as daemon */
        daemonize();
    }
}