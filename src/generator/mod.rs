use self::ast::Ast;
use self::typescript::Typescript;
use prost_types::{
    compiler::{code_generator_response::File, CodeGeneratorRequest, CodeGeneratorResponse},
    FileDescriptorProto,
};
use std::error::Error;

pub mod ast;
pub mod typescript;

const OUTPUTS: &[&Generator] = &[&Ast, &Typescript];

trait Generator {
    fn generate(&self, file: &FileDescriptorProto) -> Result<File, Box<Error>>;
}

pub fn run(req: CodeGeneratorRequest) -> Result<CodeGeneratorResponse, Box<Error>> {
    let mut files = vec![];

    for proto in req.proto_file {
        for output in OUTPUTS {
            files.push(output.generate(&proto)?);
        }
    }

    Ok(CodeGeneratorResponse {
        file: files,
        error: None,
    })
}
