#![feature(pattern, string_remove_matches)]

//! Tool that extracts signatures from a bunch of C(++) functions
//! and converts them to Rust form

mod parse;
mod transform;

use std::{env::args, fs::File};

use crate::parse::util::find_abs;

fn bulk_transform(input: &str) -> String {
    let mut out = String::new();
    let mut cursor = 0;
    loop {
        let extern_c = match find_abs(input, cursor, r#"extern "C""#) {
            Some(idx) => idx,
            None => return out,
        };
        let first_brace = find_abs(input, extern_c, '{').unwrap();
        out += &transform::transform(&input[extern_c..first_brace]);
        out.push_str(";\n");
        cursor = first_brace + 1;
    }
}

#[test]
fn test_bulk_transform() {
    use pretty_assertions::assert_eq;
    assert_eq!(
        bulk_transform(include_str!("../bulk_transform_test.cc")),
        include_str!("../bulk_transform_test.rs")
    );
}

fn main() {
    let rust_sfml_root = args().nth(1).expect("Need rust-sfml root path");
    transform_all_files(rust_sfml_root);
}

fn transform_all_files(rust_sfml_root: String) {
    use std::io::Write;
    for module in ["System", "Window", "Graphics", "Audio"] {
        let mut entries: Vec<String> =
            std::fs::read_dir(format!("{}/CSFML/src/{}", rust_sfml_root, module))
                .unwrap()
                .filter_map(|result| {
                    let name = result.unwrap().file_name().to_str().unwrap().to_owned();
                    name.ends_with(".cpp").then_some(name)
                })
                .collect();
        entries.sort();
        let mut f = File::create(format!(
            "{}/src/ffi/{}_bindgen.rs",
            rust_sfml_root,
            module.to_ascii_lowercase()
        ))
        .unwrap();
        f.write_all(
            b"// Generated by rust-sfml-bindgen\n\
                     // https://github.com/crumblingstatue/rust-sfml-bindgen\n\n\
                     unsafe extern \"C\" {\n\n",
        )
        .unwrap();
        for filename in entries {
            writeln!(f, "// {}", filename).unwrap();
            let input = std::fs::read_to_string(format!(
                "{}/CSFML/src/{}/{}",
                rust_sfml_root, module, filename
            ))
            .unwrap();
            f.write_all(bulk_transform(&input).as_bytes()).unwrap();
        }
        f.write_all(b"\n}").unwrap();
    }
}
