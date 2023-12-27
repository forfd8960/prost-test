use anyhow::{Ok, Result};
use prost::Message;
use prost_test::pb::pb as mypb;
use tokio::{io::AsyncWriteExt, net::TcpStream};

#[tokio::main]
async fn main() -> Result<()> {
    let addr = "127.0.0.1:8989";
    let mut stream = TcpStream::connect(addr).await?;

    let cmd = mypb::CommondRequest {
        request_data: Some(mypb::commond_request::RequestData::Get(mypb::Get {
            key: "test-key".to_string(),
        })),
    };

    // let mut buf = [0, 4096];
    let bs = cmd.encode_to_vec();
    println!("write: {:?} to stream", bs);
    stream.write(&bs).await?;

    println!("done write cmd: {:?}", cmd);
    Ok(())
}
