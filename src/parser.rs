use core::panic;
use std::error::Error;

use bytes::Bytes;
use tokio::io::AsyncBufReadExt;

#[derive(Debug)]
pub enum Frame {
    Bulk(String),
    Ping,
}
//pub fn get_cmmd(buf: &[u8]) {}
pub fn get_decimal(buf: &[u8]) -> Result<usize, Box<dyn Error>> {
    let decimal = buf.iter().fold(0, |acc, &x| acc * 10 + (x - 48) as usize);
    Ok(decimal)
}
impl Frame {
    pub async fn parse(buf: &[u8]) -> Result<Frame, Box<dyn Error>> {
        match buf[0] {
            b'$' => {
                let mut line_iter = buf.lines();
                let size_part = line_iter.next_line().await.unwrap().unwrap();
                let msg_size = get_decimal(size_part.as_bytes())?;
                let bulk = String::from(line_iter.next_line().await.unwrap().unwrap());
                Ok(Frame::Bulk(bulk))
            }
            _ => panic!("Failed to decode frame, doesn't match any type"),
        }
    }
}
