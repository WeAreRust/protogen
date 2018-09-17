use crate::generator::{Generator, Output};
use itertools::Itertools;
use protobuf::descriptor::{EnumDescriptorProto, EnumValueDescriptorProto, FileDescriptorProto};

pub struct Typescript;

impl Generator for Typescript {
    fn generate(fd: &FileDescriptorProto) -> Output {
        let enums = gen_enum(fd.get_enum_type());

        Output {
            name: format!("{}.ts", fd.get_name()),
            content: format!("// Generated from {}\n\n{}", &fd.get_name(), enums),
        }
    }
}

fn gen_enum(proto: &[EnumDescriptorProto]) -> String {
    proto
        .iter()
        .map(|proto| {
            format!(
                "export const enum {} {{\n\t{},\n}}",
                proto.get_name(),
                gen_enum_value(proto.get_value()),
            )
        })
        .join("\n\n")
}

fn gen_enum_value(proto: &[EnumValueDescriptorProto]) -> String {
    proto.iter().map(|value| value.get_name()).join(",\n\t")
}
