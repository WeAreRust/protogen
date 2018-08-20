// extern crate protobuf;
extern crate prost;
extern crate prost_types;

use std::io::{self, Read, Write};
use prost::Message;
use prost_types::compiler::{CodeGeneratorRequest, CodeGeneratorResponse};
use prost_types::compiler::code_generator_response::File;

fn main() {
    let mut buf = vec![];
    let _ = io::stdin()
        .read_to_end(&mut buf)
        .expect("Could not read stdin");

    let req = CodeGeneratorRequest::decode(buf);

    let res = match req {
        Result::Err(e) => CodeGeneratorResponse {
            error: Some(format!("Error: {:?}", e)),
            file: vec![],
        },
        Result::Ok(r) => CodeGeneratorResponse {
            error: None,
            file: vec![File {
                name: Some(String::from("output.txt")),
                insertion_point: None,
                content: Some(format!("{:#?}", r)),
            }],
        },
    };

    let mut outbuf = vec![];
    res.encode(&mut outbuf).expect("Could not encode output");

    io::stdout().write_all(&outbuf).expect("Could not write output");
}
