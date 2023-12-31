use anyhow::{Ok, Result};
use bytes::BytesMut;
use prost::Message;
use prost_test::pb::pb as mypb;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

#[tokio::main]
async fn main() -> Result<()> {
    let addr = "127.0.0.1:8989";
    let mut stream = TcpStream::connect(addr).await?;

    let mut cmds: Vec<Vec<u8>> = vec![];

    let get_cmd = encode_get_cmd();
    cmds.push(get_cmd);

    let set_cmd = encode_set_cmd();
    cmds.push(set_cmd);

    for data in cmds {
        stream.write_all(&data).await.unwrap();
        println!("done write cmd: {:?}", data);

        let mut resp = BytesMut::with_capacity(4096);
        stream.read_buf(&mut resp).await.unwrap();
        println!("read response: {:?}", String::from_utf8(resp.to_vec()));
    }

    Ok(())
}

fn encode_get_cmd() -> Vec<u8> {
    let cmd = mypb::CommondRequest {
        request_data: Some(mypb::commond_request::RequestData::Get(mypb::Get {
            key: "test-key".to_string(),
        })),
    };

    // let mut buf = [0, 4096];
    let bs = cmd.encode_to_vec();
    bs
}

fn encode_set_cmd() -> Vec<u8> {
    let cmd = mypb::CommondRequest {
        request_data: Some(mypb::commond_request::RequestData::Set(mypb::Set {
            kv: Some(mypb::Kv {
                key: "test-key".to_string(),
                value: "test-value".to_string(),
            }),
        })),
    };

    // let mut buf = [0, 4096];
    let bs = cmd.encode_to_vec();
    bs
}
