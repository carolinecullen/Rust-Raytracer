use nom::*;
/// Parser written using nom which a zero-copy and byte oriented parser for rust.
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

named!(
    string_between_braces,
    delimited!(char!('{'), is_not!("}"), char!('}'))
);
named!(get_string, take_while!(is_alphabetic));
named!(remove_whitespace, take_while!(is_whitespace));
named!(camera_parser, tag!("camera"));
named!(light_source_parser, tag!("light_source"));
named!(plane_parser, tag!("plane"));
named!(sphere_parser, tag!("sphere"));

fn is_whitespace(c: u8) -> bool {
    c as char == ' ' || c as char == '\t'
}

pub fn parse(file_name: &str) -> Result<(), &str> {
    let mut tests_file = "../tests/".to_owned();
    tests_file.push_str(file_name);
    let file_path = Path::new(&tests_file);
    let mut f = File::open(file_path).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    parse_for_elements(contents.as_bytes());

    Ok(())
}

fn parse_for_elements(contents: &[u8]) {
    // let (i, cell) = try_parse!(
    //     contents,
    //     preceded!(
    //         opt!(remove_whitespace),
    //         alt!(camera_parser | light_source_parser | plane_parser | sphere_parser)
    //     )
    // ).unwrap();
}
