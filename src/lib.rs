mod classfile;
mod class_builder;
mod java_type_signatures;
mod pretty_printing;
mod serialization;

use std::fs::File;
use std::io::Write;

pub use classfile::*;
pub use class_builder::*;
pub use java_type_signatures::*;

pub fn write_classfile(classfile: Classfile, filename: &str) {
    let mut bytes = vec![];
    classfile.serialize(&mut bytes);

    let mut f = File::create(filename).unwrap();
    f.write_all(&bytes).unwrap();
}

pub fn read_classfile(filename: &str) -> Classfile {
    let f = File::open(filename).unwrap();
    Classfile::deserialize(Box::new(f))
}

pub fn define_class(access_flags: u16, this_class: &str, super_class: &str) -> ClassBuilder {
    ClassBuilder::new(access_flags, this_class, super_class)
}
