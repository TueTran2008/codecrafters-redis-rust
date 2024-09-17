#![allow(unused_imports, unused_variables)]
use std::{error::Error, fmt::format};

use anyhow::Ok;

use bytes::{Buf, BytesMut};
use std::str;
use tokio::{
    io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};
mod parser;
//use redis_starter_rust::parser;
async fn process_socket(mut stream: TcpStream) -> Result<(), Box<dyn Error>> {
    let ping = "hehe";
    let pong = "+PONG\r\n";
    let echo = "*2\r\n$4\r\nECHO\r\n";
    let mut buf = [0; 128];
    //println!("Accpet a connection: {:?}", socket);
    loop {
        stream.readable().await?;
        let number_of_bytes_read = stream.read(&mut buf).await?;
        println!("read bytes {number_of_bytes_read} {:?}", buf);
        let mut line_iter = buf.lines();
        let size_str = line_iter.next_line().await.unwrap().unwrap();
        println!("size str {size_str}");
        let array_size = parser::get_decimal(&size_str.as_bytes()[1..])?;
        println!("array size {array_size}");
        let _ = line_iter.next_line().await.unwrap().unwrap();
        if array_size == 2 {
            println!("hehe");
            let cmd = line_iter.next_line().await.unwrap().unwrap();
            println!("lalalal{cmd}");
            if cmd.starts_with("ECHO") {
                let data_size = line_iter.next_line().await.unwrap().unwrap();
                let data = line_iter.next_line().await.unwrap().unwrap();
                let reponse = format!("{data_size}\r\n{data}\r\n");
                println!("{reponse}");
                stream.write_all(reponse.as_bytes()).await?;
            } else {
                print!("{cmd}");
            }
        } else {
            stream.write_all(pong.as_bytes()).await?;
        }
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
            let _ = process_socket(socket).await;
        });
    }
}
