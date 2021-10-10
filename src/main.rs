#![feature(pattern)]

//! Tool that extracts signatures from a bunch of C(++) functions
//! and converts them to Rust form

use crate::raw::parse_sig_raw;

mod raw;
#[derive(Debug, PartialEq)]
struct Signature<'a> {
    name: &'a str,
    ret_type: Type<'a>,
    args: Vec<Arg<'a>>,
}
#[derive(Debug, PartialEq)]
struct Type<'a> {
    ident: &'a str,
    pointer: Ptr,
}
#[derive(Debug, PartialEq)]
enum Ptr {
    No,
    Const,
    Mut,
}

#[derive(Debug, PartialEq)]
struct Arg<'a> {
    name: &'a str,
    type_: Type<'a>,
}

fn parse(input: &str) -> Signature {
    let raw = parse_sig_raw(input);
    let (fname, fty) = raw_ptr_conv(raw.name, raw.ret_type);
    let mut args = Vec::new();
    for arg in raw.args {
        let (aname, aty) = raw_ptr_conv(arg.name, arg.type_);
        args.push(Arg {
            name: aname,
            type_: aty,
        });
    }
    Signature {
        name: fname,
        ret_type: fty,
        args,
    }
}

fn raw_ptr_conv<'a>(mut name: &'a str, mut type_: &'a str) -> (&'a str, Type<'a>) {
    dbg!(name, type_);
    let mut const_ = false;
    let mut ptr = false;
    if name.contains("const") {
        const_ = true;
        name = name.trim_end_matches("const");
    }
    if name.contains('*') {
        ptr = true;
        name = name.trim_matches('*');
    }
    if type_.contains("const") {
        const_ = true;
        type_ = type_.trim_start_matches("const");
    }
    if type_.contains('*') {
        ptr = true;
        type_ = type_.trim_matches('*');
    }
    let pointer = match (ptr, const_) {
        (true, false) => Ptr::Mut,
        (true, true) => Ptr::Const,
        (false, _) => Ptr::No,
    };
    (
        name.trim(),
        Type {
            ident: type_.trim(),
            pointer,
        },
    )
}

#[test]
fn test_parse() {
    use Ptr::*;
    assert_eq!(
        parse(
            r#"extern "C" sfVector2i sfTouch_getPositionRenderWindow(unsigned int finger, const sfRenderWindow *relativeTo)"#
        ),
        Signature {
            name: "sfTouch_getPositionRenderWindow",
            ret_type: Type {
                ident: "sfVector2i",
                pointer: No,
            },
            args: vec![
                Arg {
                    name: "finger",
                    type_: Type {
                        ident: "unsigned int",
                        pointer: No,
                    },
                },
                Arg {
                    name: "relativeTo",
                    type_: Type {
                        ident: "sfRenderWindow",
                        pointer: Const,
                    },
                },
            ],
        }
    );
    assert_eq!(
        parse(
            r#"extern "C" void sfRenderWindow_drawPrimitives(sfRenderWindow *renderWindow,
        const sfVertex *vertices, size_t vertexCount,
        sfPrimitiveType type, const sf::RenderStates *states) {"#
        ),
        Signature {
            name: "sfRenderWindow_drawPrimitives",
            ret_type: Type {
                ident: "void",
                pointer: No,
            },
            args: vec![
                Arg {
                    name: "renderWindow",
                    type_: Type {
                        ident: "sfRenderWindow",
                        pointer: Mut,
                    },
                },
                Arg {
                    name: "vertices",
                    type_: Type {
                        ident: "sfVertex",
                        pointer: Const,
                    },
                },
                Arg {
                    name: "vertexCount",
                    type_: Type {
                        ident: "size_t",
                        pointer: No,
                    },
                },
                Arg {
                    name: "type",
                    type_: Type {
                        ident: "sfPrimitiveType",
                        pointer: No,
                    },
                },
                Arg {
                    name: "states",
                    type_: Type {
                        ident: "sf::RenderStates",
                        pointer: Const,
                    },
                },
            ],
        }
    );
}

fn main() {
    dbg!(parse(
        r#"extern "C" void sfRenderWindow_drawPrimitives(sfRenderWindow *renderWindow,
        const sfVertex *vertices, size_t vertexCount,
        sfPrimitiveType type, const sf::RenderStates *states) "#
    ));
}
