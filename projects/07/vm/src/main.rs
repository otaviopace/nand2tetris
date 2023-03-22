use std::env;
use std::ffi::OsStr;
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use vm::parser::Parser;

/// Removes extension from file name.
fn file_name(file: &str) -> Option<String> {
    Path::new(file)
        .file_name()
        .and_then(OsStr::to_str)
        .map(|name| name.chars().take_while(|ch| *ch != '.').collect())
}

/// The vm converts the vm byte code into Hack Assembly
fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let file_or_dir = &args[0];
    let metadata = fs::metadata(file_or_dir).unwrap();

    if metadata.is_file() {
        let file = file_or_dir;

        let program = fs::read_to_string(file.clone()).expect("vm program doesn't exist");

        let instructions = Parser::parse(program.lines());

        let mut new_file = PathBuf::from(file);

        new_file.set_extension("asm");

        let mut translated_file = File::create(new_file).expect("failed to create assembly file");

        let file_name = file_name(file).unwrap();

        for instruction in instructions {
            translated_file
                .write_all(instruction.to_assembly(&file_name).as_bytes())
                .expect("failed to write instruction to translated file");

            translated_file
                .write_all(b"\n")
                .expect("failed to write new line to translated file");
        }
    } else if metadata.is_dir() {
        // TODO: handle dir of vm files
    } else {
        panic!("unsupported parameter, passing symlink maybe?");
    }
}
