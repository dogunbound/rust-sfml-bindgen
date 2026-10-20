pub mod util;

use self::raw::parse_sig_raw;

mod raw;

#[derive(Debug, PartialEq)]
pub(crate) struct Signature<'a> {
    pub(crate) name: &'a str,
    pub(crate) ret_type: Type<'a>,
    pub(crate) args: Vec<Arg<'a>>,
}
#[derive(Debug, PartialEq)]
pub(crate) struct Type<'a> {
    pub(crate) ident: &'a str,
    pub(crate) pointer: Ptr,
}
#[derive(Debug, PartialEq)]
pub(crate) enum Ptr {
    No,
    Const,
    Mut,
    /// Two levels of indirection, such as `const char *const *`.
    /// `outer_const` and `inner_const` give the constness of the outer and inner pointer.
    Double { outer_const: bool, inner_const: bool },
}

#[derive(Debug, PartialEq)]
pub(crate) struct Arg<'a> {
    pub(crate) name: &'a str,
    pub(crate) type_: Type<'a>,
}

pub(crate) fn parse(input: &str) -> Signature {
    //    eprintln!("=== Parsing {} ===", input);
    let raw = parse_sig_raw(input);
    // dbg!(&raw);
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
    //    dbg!(name, type_);
    let name_stars = name.matches('*').count();
    let type_stars = type_.matches('*').count();
    if name_stars + type_stars == 2 && type_stars >= 1 {
        let parts: Vec<&'a str> = type_.split('*').collect();
        return (
            name.trim_matches('*').trim(),
            Type {
                ident: parts[0]
                    .trim_start_matches("const")
                    .trim_end_matches("const")
                    .trim(),
                pointer: Ptr::Double {
                    outer_const: parts[1].contains("const"),
                    inner_const: parts[0].contains("const"),
                },
            },
        );
    }
    assert!(
        name_stars + type_stars < 2,
        "unsupported pointer depth in `{name} {type_}`"
    );
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
        type_ = type_.trim_end_matches("const");
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
    use pretty_assertions::assert_eq;
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
    assert_eq!(
        parse(
            r#"extern "C" const sfView *sfRenderWindow_getView(const sfRenderWindow *renderWindow)"#
        ),
        Signature {
            name: "sfRenderWindow_getView",
            args: vec![Arg {
                name: "renderWindow",
                type_: Type {
                    ident: "sfRenderWindow",
                    pointer: Const,
                }
            }],
            ret_type: Type {
                ident: "sfView",
                pointer: Const
            }
        }
    );
    assert_eq!(
        parse(r#"extern "C" sf::Transform const *sfSprite_getTransform(const sf::Sprite *sprite)"#),
        Signature {
            name: "sfSprite_getTransform",
            args: vec![Arg {
                name: "sprite",
                type_: Type {
                    ident: "sf::Sprite",
                    pointer: Const
                }
            }],
            ret_type: Type {
                ident: "sf::Transform",
                pointer: Const
            }
        }
    )
}

#[test]
fn test_parse_double_pointer() {
    use pretty_assertions::assert_eq;
    use Ptr::*;
    assert_eq!(
        parse(
            r#"extern "C" const char *const *sfVulkan_getGraphicsRequiredInstanceExtensions(size_t *count)"#
        ),
        Signature {
            name: "sfVulkan_getGraphicsRequiredInstanceExtensions",
            ret_type: Type {
                ident: "char",
                pointer: Double {
                    outer_const: true,
                    inner_const: true,
                },
            },
            args: vec![Arg {
                name: "count",
                type_: Type {
                    ident: "size_t",
                    pointer: Mut,
                },
            }],
        }
    )
}
