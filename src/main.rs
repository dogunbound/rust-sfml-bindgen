#![feature(pattern, string_remove_matches)]

//! Tool that extracts signatures from a bunch of C(++) functions
//! and converts them to Rust form

use parse::Ptr;

mod parse;

fn transform(input: &str) -> String {
    let sig = parse::parse(input);
    let args = transform_args(sig.args);
    let ret = transform_ret(sig.ret_type);
    format!("pub fn {}({}){}", ident(sig.name), args, ret)
}

fn transform_ret(type_: parse::Type) -> String {
    if type_.pointer == Ptr::No && type_.ident == "void" {
        String::new()
    } else {
        format!(" -> {}", transform_type(type_))
    }
}

fn transform_args(args: Vec<parse::Arg>) -> String {
    let mut out = String::new();
    for arg in args {
        out += &format!("{}, ", transform_arg(arg));
    }
    // Remove last `, `
    out.pop();
    out.pop();
    out
}

fn transform_arg(arg: parse::Arg) -> String {
    format!("{}: {}", ident(arg.name), transform_type(arg.type_))
}

fn transform_type(type_: parse::Type) -> String {
    let p = match type_.pointer {
        Ptr::No => "",
        Ptr::Const => "*const ",
        Ptr::Mut => "*mut ",
    };
    format!("{}{}", p, ident(type_.ident))
}

fn ident(input: &str) -> String {
    let mut out = String::from(input);
    match input {
        "as" | "break" | "const" | "continue" | "crate" | "else" | "enum" | "extern" | "false"
        | "fn" | "for" | "if" | "impl" | "in" | "let" | "loop" | "match" | "mod" | "move"
        | "mut" | "pub" | "ref" | "return" | "self" | "Self" | "static" | "struct" | "super"
        | "trait" | "true" | "type" | "unsafe" | "use" | "where" | "while" | "abstract"
        | "become" | "box" | "do" | "final" | "macro" | "override" | "priv" | "typeof"
        | "unsized" | "virtual" | "yield" | "try" => out.push('_'),
        _ => {}
    }
    out.remove_matches("::");
    out
}

#[test]
fn test_transform() {
    assert_eq!(
        transform(
            r#"extern "C" void sfRenderWindow_drawPrimitives(sfRenderWindow *renderWindow,
        const sfVertex *vertices, size_t vertexCount,
        sfPrimitiveType type, const sf::RenderStates *states) "#
        ),
        "pub fn sfRenderWindow_drawPrimitives(\
            renderWindow: *mut sfRenderWindow, \
            vertices: *const sfVertex, \
            vertexCount: size_t, \
            type_: sfPrimitiveType, \
            states: *const sfRenderStates)"
    );
    assert_eq!(
        transform(
            r#"extern "C" sf::WindowHandle sfRenderWindow_getSystemHandle(const sfRenderWindow *renderWindow) {"#
        ),
        "pub fn sfRenderWindow_getSystemHandle(renderWindow: *const sfRenderWindow) -> sfWindowHandle"
    )
}

fn main() {}
