use anyhow::Result;
use bytes::Bytes;
use prost::Message;
use tokio::{io::AsyncWriteExt, net::TcpListener};

use prost_test::pb::pb as mypb;

#[tokio::main]
async fn main() -> Result<()> {
    let addr: &str = "127.0.0.1:8989";
    let listener = TcpListener::bind(addr).await?;
    println!("start listen on: {}", addr);

    loop {
        let (mut tcp_stream, addr) = listener.accept().await?;
        println!("client: {} connected", addr);

        let _ = tcp_stream.readable().await;

        tokio::spawn(async move {
            loop {
                let mut buf = [0; 4096];
                match tcp_stream.try_read(&mut buf) {
                    Ok(0) => {
                        continue;
                    }

                    Ok(n) => {
                        println!("read {} bytes", n);
                        let data = Bytes::copy_from_slice(&buf[0..n]);
                        let req_data = mypb::CommondRequest::decode(data);
                        match req_data {
                            Ok(req) => {
                                println!("decode cmd: {:?}", req);
                                let _ = tcp_stream.write_all(b"Received data").await;
                            }
                            Err(e) => {
                                println!("failed to decode request: {}", e);
                            }
                        }
                        continue;
                    }
                    Err(ref e) => {
                        println!("read data err: {}", e);
                        continue;
                    }
                }
            }
        });
    }
}
