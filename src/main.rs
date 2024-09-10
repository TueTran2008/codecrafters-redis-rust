#![allow(unused_imports, unused_variables)]
use anyhow::Ok;
//#[tokio::main]
//use mini_redis::{client, Result};
//use std::{io, net::TcpListener};
use tokio::{
    io::AsyncWriteExt,
    net::{TcpListener, TcpStream},
};

async fn process_socket(mut socket: TcpStream) {
    let pong = "+PONG\r\n";
    //println!("Accpet a connection: {:?}", socket);
    let _ = socket.write_all(pong.as_bytes()).await;
}

#[tokio::main()]
async fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();
    loop {
        let (socket, _) = listener.accept().await.unwrap();
        process_socket(socket).await;
    }
}
