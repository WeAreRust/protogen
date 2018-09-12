use crate::generator::Generator;
use prost_types::{compiler::code_generator_response::File, FileDescriptorProto};
use std::{error::Error, io};

pub struct Ast;

impl Generator for Ast {
    fn generate(fd: &FileDescriptorProto) -> Result<File, Box<Error>> {
        let name = format!(
            "{}.ast.txt",
            fd.name.clone().ok_or(io::Error::new(
                io::ErrorKind::Other,
                "no filename for proto file",
            ))?
        );

        let content = format!("{:#?}", fd);

        Ok(File {
            name: Some(name),
            content: Some(content),
            ..Default::default()
        })
    }
}
