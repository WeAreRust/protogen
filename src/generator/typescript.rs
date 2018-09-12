use super::Generator;
use prost_types::compiler::code_generator_response::File;
use prost_types::FileDescriptorProto;
use std::error::Error;

pub struct Typescript;

impl Generator for Typescript {
    fn generate(&self, file: &FileDescriptorProto) -> Result<File, Box<Error>> {
        let filename = file.name.clone().ok_or("no proto filename")?;

        let enums: Vec<String> = file
            .enum_type
            .iter()
            .map(|v| {
                ts_enum(
                    v.name.clone().unwrap(),
                    v.value.iter().map(|v| v.name.clone().unwrap()).collect(),
                )
            })
            .collect();
        Ok(File {
            name: Some(format!("{}.ts", filename)),
            insertion_point: None,
            content: Some(format!(
                "// Generated from {}\n\n{}",
                filename,
                enums.join("\n\n")
            )),
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
