use crate::generator::{Generator, Output};
use protobuf::descriptor::FileDescriptorProto;

pub struct Ast;

impl Generator for Ast {
    fn generate(fd: &FileDescriptorProto) -> Output {
        Output {
            name: format!("{}.ast.txt", fd.get_name()),
            content: format!("{:#?}", fd),
        }
    }
}
