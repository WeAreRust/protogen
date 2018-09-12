use self::{ast::Ast, typescript::Typescript};
use prost_types::{
    compiler::{code_generator_response::File, CodeGeneratorRequest, CodeGeneratorResponse},
    FileDescriptorProto,
};
use std::error::Error;

pub mod ast;
pub mod typescript;

trait Generator {
    fn generate(fd: &FileDescriptorProto) -> Result<File, Box<Error>>;
}

pub fn run(req: CodeGeneratorRequest) -> Result<CodeGeneratorResponse, Box<Error>> {
    let mut files = vec![];
    for proto in req.proto_file {
        files.push(Ast::generate(&proto)?);
        files.push(Typescript::generate(&proto)?);
    }

    Ok(CodeGeneratorResponse {
        file: files,
        ..Default::default()
    })
}
