#![feature(box_syntax)]

mod generator;

use prost::Message;
use prost_types::compiler::{CodeGeneratorRequest, CodeGeneratorResponse};
use std::{
    error::Error,
    io::{self, Read, Write},
};

fn main() -> Result<(), Box<dyn Error>> {
    get_protoc_request()
        .and_then(|req| generator::run(req))
        .or_else(|e| {
            Ok(CodeGeneratorResponse {
                error: Some(e.description().into()),
                ..Default::default()
            })
        })
        .and_then(|resp| send_protoc_response(resp))
}

fn get_protoc_request() -> Result<CodeGeneratorRequest, Box<Error>> {
    let mut message = vec![];
    io::stdin().read_to_end(&mut message)?;
    CodeGeneratorRequest::decode(message).map_err(|e| e.into())
}

fn send_protoc_response(resp: CodeGeneratorResponse) -> Result<(), Box<Error>> {
    let mut message = vec![];
    resp.encode(&mut message)?;
    io::stdout().write_all(&message).map_err(|e| e.into())
}
