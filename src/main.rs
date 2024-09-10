#![allow(unused_imports, unused_variables)]
use std::error::Error;

use anyhow::Ok;

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

async fn process_socket(mut stream: TcpStream) -> Result<(), Box<dyn Error>> {
    let pong = "+PONG\r\n";
    let mut buf = [0; 4096];
    //println!("Accpet a connection: {:?}", socket);
    loop {
        stream.readable().await?;
        stream.read(&mut buf).await?;
        let _ = stream.write_all(pong.as_bytes()).await;
        //println!("hehe: {:?}", hehe);
    }
}

#[tokio::main()]
async fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();
    loop {
        let (socket, _) = listener.accept().await.unwrap();
        tokio::spawn(async move {
            process_socket(socket).await;
        });
    }
}
