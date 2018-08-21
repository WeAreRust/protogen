use prost_types::compiler::{
    code_generator_response::File,
    {CodeGeneratorRequest, CodeGeneratorResponse},
};
use prost_types::FileDescriptorProto;
use std::error::Error;

pub fn handler(req: CodeGeneratorRequest) -> Result<CodeGeneratorResponse, Box<Error>> {
    let mut files = vec![];
    for proto_file in req.proto_file {
        files.push(ts_file(&proto_file)?);
        files.push(ast_file(&proto_file)?);
    }
    Ok(CodeGeneratorResponse {
        file: files,
        error: None,
    })
}

fn ast_file(file: &FileDescriptorProto) -> Result<File, Box<Error>> {
    let filename = file.name.clone().ok_or("no proto filename")?;
    Ok(File {
        name: Some(format!("{}.ast.txt", filename)),
        insertion_point: None,
        content: Some(format!("{:#?}", file)),
    })
}

fn ts_file(file: &FileDescriptorProto) -> Result<File, Box<Error>> {
    let filename = file.name.clone().ok_or("no proto filename")?;
    Ok(File {
        name: Some(format!("{}.ts", filename)),
        insertion_point: None,
        content: Some(format!("// Generated from {}", filename)),
    })
}
