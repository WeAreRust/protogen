use crate::generator::Generator;
use prost_types::{compiler::code_generator_response::File, FileDescriptorProto};
use std::{error::Error, io};

pub struct Typescript;

impl Generator for Typescript {
    fn generate(fd: &FileDescriptorProto) -> Result<File, Box<Error>> {
        let name = format!(
            "{}.ts",
            fd.name.clone().ok_or(io::Error::new(
                io::ErrorKind::Other,
                "no filename for proto file",
            ))?
        );

        let enums: Vec<String> = fd
            .enum_type
            .iter()
            .map(|v| {
                ts_enum(
                    v.name.clone().unwrap(),
                    v.value.iter().map(|v| v.name.clone().unwrap()).collect(),
                )
            })
            .collect();
        let content = format!("// Generated from {}\n\n{}", &name, enums.join("\n\n"));

        Ok(File {
            name: Some(name),
            content: Some(content),
            ..Default::default()
        })
    }
}

fn ts_enum(name: String, values: Vec<String>) -> String {
    format!(
        "export const enum {} {{\n\t{},\n}}",
        name,
        values.join(",\n\t")
    )
}
