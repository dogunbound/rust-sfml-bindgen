#![feature(pattern, string_remove_matches)]

//! Tool that extracts signatures from a bunch of C(++) functions
//! and converts them to Rust form

mod parse;
mod transform;

use std::env::args;

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
    let csfml_src_path = args().nth(1).expect("Need path to CSFML/src");
    transform_all_files(csfml_src_path);
}

fn transform_all_files(csfml_src_path: String) {
    for module in ["System", "Window", "Graphics", "Audio"] {
        println!("== {} ==", module);
        let mut entries: Vec<String> = std::fs::read_dir(format!("{}/{}", csfml_src_path, module))
            .unwrap()
            .filter_map(|result| {
                let name = result.unwrap().file_name().to_str().unwrap().to_owned();
                name.ends_with(".cpp").then_some(name)
            })
            .collect();
        entries.sort();
        for filename in entries {
            println!("// {}", filename);
            let input =
                std::fs::read_to_string(format!("{}/{}/{}", csfml_src_path, module, filename))
                    .unwrap();
            println!("{}", bulk_transform(&input));
        }
    }
}
