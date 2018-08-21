extern crate prost;
extern crate prost_types;

use prost::Message;
use prost_types::compiler::{
    code_generator_response::File,
    {CodeGeneratorRequest, CodeGeneratorResponse},
};
use std::{
    error::Error,
    io::{stdin, stdout, Read, Write},
};

fn main() -> Result<(), Box<Error>> {
    let mut buf = vec![];
    stdin().read_to_end(&mut buf).map(|_| ())?;

    let req = CodeGeneratorRequest::decode(buf);
    let res = match req {
        Ok(r) => CodeGeneratorResponse {
            file: vec![File {
                name: Some("output.txt".into()),
                content: Some(format!("{:#?}", r)),
                ..Default::default()
            }],
            ..Default::default()
        },
        Err(e) => CodeGeneratorResponse {
            error: Some(e.description().into()),
            ..Default::default()
        },
    };

    let mut outbuf = vec![];
    res.encode(&mut outbuf)?;
    stdout().write_all(&outbuf)?;

    Ok(())
}
