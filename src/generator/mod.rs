use self::{ast::Ast, typescript::Typescript};
use protobuf::{
    descriptor::FileDescriptorProto,
    plugin::{CodeGeneratorRequest, CodeGeneratorResponse, CodeGeneratorResponse_File},
    RepeatedField,
};
use std::str;

pub mod ast;
pub mod typescript;

pub struct Output {
    pub name: String,
    pub content: String,
}

impl From<Output> for CodeGeneratorResponse_File {
    fn from(gen: Output) -> Self {
        let mut resp_file = CodeGeneratorResponse_File::new();
        resp_file.set_name(gen.name.into());
        resp_file.set_content(str::from_utf8(gen.content.as_ref()).unwrap().into());
        resp_file
    }
}

trait Generator {
    fn generate(fd: &FileDescriptorProto) -> Output;
}

pub fn run(req: CodeGeneratorRequest) -> CodeGeneratorResponse {
    let mut files = vec![];
    for proto in req.get_proto_file() {
        files.push(Ast::generate(&proto).into());
        files.push(Typescript::generate(&proto).into());
    }

    let mut resp = CodeGeneratorResponse::new();
    resp.set_file(RepeatedField::from_vec(files));
    resp
}
