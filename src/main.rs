mod pb;

use crate::pb::pb as mypb;

fn main() {
    let kv = mypb::Kv {
        key: "Hello".to_string(),
        value: "World!".to_string(),
    };

    let kv1 = mypb::Kv {
        key: "Hello".to_string(),
        value: "World!".to_string(),
    };
    println!("{:?}", kv);
    println!("kv == kv1: {:?}", kv == kv1);
}
