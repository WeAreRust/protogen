extern crate prost;
extern crate prost_types;

mod generators;

use generators::typescript;
use prost::Message;
use prost_types::compiler::{CodeGeneratorRequest, CodeGeneratorResponse};
use std::{
    error::Error,
    io::{stdin, stdout, Read, Write},
};

fn main() -> Result<(), Box<Error>> {
    let mut buf = vec![];
    stdin().read_to_end(&mut buf).map(|_| ())?;

    let req: Result<CodeGeneratorRequest, Box<Error>> =
        CodeGeneratorRequest::decode(buf).map_err(|e| Box::from(e));

    let res: CodeGeneratorResponse = req
        .and_then(|v| typescript::handler(v))
        .unwrap_or_else(|e| CodeGeneratorResponse {
            error: Some(e.description().into()),
            ..Default::default()
        });

    let mut outbuf = vec![];
    res.encode(&mut outbuf)?;
    stdout().write_all(&outbuf)?;

    Ok(())
}
