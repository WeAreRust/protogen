use super::Generator;
use prost_types::compiler::code_generator_response::File;
use prost_types::FileDescriptorProto;
use std::error::Error;

pub struct Ast;

impl Generator for Ast {
    fn generate(&self, file: &FileDescriptorProto) -> Result<File, Box<Error>> {
        let filename = file.name.clone().ok_or("no proto filename")?;
        Ok(File {
            name: Some(format!("{}.ast.txt", filename)),
            insertion_point: None,
            content: Some(format!("{:#?}", file)),
        })
    }
}
