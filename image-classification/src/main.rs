mod infer;
use std::io::{self, Read};

pub fn main() {
    let mut buf = Vec::new();
    io::stdin().read_to_end(&mut buf).unwrap();
    let res = infer::infer_internal(&buf);
    println!("{}", res);
}
