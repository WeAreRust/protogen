mod generator;

use protobuf::{
    error::ProtobufError,
    plugin::{CodeGeneratorRequest, CodeGeneratorResponse},
    Message,
};
use std::{error::Error, io};

fn main() -> Result<(), Box<dyn Error>> {
    get_protoc_request()
        .and_then(|req| Ok(generator::run(req)))
        .or_else(|e| {
            let mut resp = CodeGeneratorResponse::new();
            resp.set_error(e.description().into());
            Ok(resp)
        })
        .and_then(|resp| send_protoc_response(resp))
        .map_err(|e| e.into())
}

fn get_protoc_request() -> Result<CodeGeneratorRequest, ProtobufError> {
    protobuf::parse_from_reader(&mut io::stdin())
}

fn send_protoc_response(resp: CodeGeneratorResponse) -> Result<(), ProtobufError> {
    resp.write_to_writer(&mut io::stdout())
}
