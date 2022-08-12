use crate::parse::util::{find_abs, rfind_abs};

#[derive(Debug, PartialEq)]
pub(super) struct RawSig<'a> {
    pub(super) ret_type: &'a str,
    pub(super) name: &'a str,
    pub(super) args: Vec<RawArg<'a>>,
}

#[derive(Debug, PartialEq)]
pub(super) struct RawArg<'a> {
    pub(super) type_: &'a str,
    pub(super) name: &'a str,
}

fn find_arg_delim_right(input: &str) -> usize {
    input.find(&[',', ')'][..]).unwrap_or(input.len())
}

fn find_arg_delim_left_rev(input: &str) -> usize {
    input.rfind(&[',', '('][..]).unwrap_or(0)
}

fn parse_arg_raw(input: &str) -> RawArg {
    let name_end = find_arg_delim_right(input);
    let name_start = input[..name_end]
        .rfind(|c: char| c.is_whitespace())
        .unwrap_or(0);
    let type_start = find_arg_delim_left_rev(&input[..name_start]);
    RawArg {
        type_: input[type_start..name_start].trim(),
        name: input[name_start..name_end].trim(),
    }
}

pub(super) fn parse_sig_raw(mut input: &str) -> RawSig {
    input = input.trim_start_matches(r#"extern "C""#);
    input = input.trim_start();
    //dbg!(input);
    let args_open = find_abs(input, 0, '(').unwrap();
    //dbg!(&input[0..args_open]);
    let first_space_rev = rfind_abs(input, args_open, |c: char| c.is_whitespace()).unwrap();
    let fname = &input[first_space_rev..args_open];
    //dbg!(fname);
    let type_ = &input[..first_space_rev - 1];
    //dbg!(type_);
    let args_close = find_abs(input, args_open + 1, ')').unwrap();
    let args_part = &input[args_open + 1..args_close];
    //dbg!(args_part);
    let args = parse_args_raw(args_part);
    RawSig {
        name: fname,
        ret_type: type_,
        args,
    }
}

fn parse_args_raw(input: &str) -> Vec<RawArg> {
    //    dbg!(input);
    input
        .split(',')
        .filter_map(|chunk| {
            let arg = parse_arg_raw(chunk);
            if arg.name.is_empty() || arg.type_.is_empty() {
                None
            } else {
                Some(arg)
            }
        })
        .collect()
}

#[test]
fn test_parse_arg_raw() {
    assert_eq!(
        parse_arg_raw("unsigned long foo, "),
        RawArg {
            type_: "unsigned long",
            name: "foo"
        }
    );
    assert_eq!(
        parse_arg_raw("sf::IntRect *bar)"),
        RawArg {
            type_: "sf::IntRect",
            name: "*bar"
        }
    );
    assert_eq!(
        parse_arg_raw("const sf::IntRect* baz)"),
        RawArg {
            type_: "const sf::IntRect*",
            name: "baz"
        }
    );
}

#[test]
fn test_parse_sig_raw() {
    use pretty_assertions::assert_eq;
    assert_eq!(
        parse_sig_raw(
            r#"extern "C" sfVector2i sfTouch_getPositionRenderWindow(unsigned int finger, const sfRenderWindow *relativeTo)"#
        ),
        RawSig {
            ret_type: "sfVector2i",
            name: "sfTouch_getPositionRenderWindow",
            args: vec![
                RawArg {
                    name: "finger",
                    type_: "unsigned int",
                },
                RawArg {
                    name: "*relativeTo",
                    type_: "const sfRenderWindow",
                },
            ]
        }
    );
    assert_eq!(
        parse_sig_raw(
            r#"extern "C" void sfMouse_setPositionRenderWindow(sfVector2i position, const sfRenderWindow *relativeTo)"#
        ),
        RawSig {
            ret_type: "void",
            name: "sfMouse_setPositionRenderWindow",
            args: vec![
                RawArg {
                    name: "position",
                    type_: "sfVector2i"
                },
                RawArg {
                    name: "*relativeTo",
                    type_: "const sfRenderWindow",
                }
            ]
        }
    );
    assert_eq!(
        parse_sig_raw(
            r#"extern "C" const sfView *sfRenderWindow_getView(const sfRenderWindow *renderWindow)"#
        ),
        RawSig {
            ret_type: "const sfView",
            name: "*sfRenderWindow_getView",
            args: vec![RawArg {
                name: "*renderWindow",
                type_: "const sfRenderWindow"
            }]
        }
    );
    // Single void arg
    assert_eq!(
        parse_sig_raw(r#"extern "C" sf::Context *sfContext_create(void)"#),
        RawSig {
            ret_type: "sf::Context",
            name: "*sfContext_create",
            args: vec![],
        }
    );
    // Empty arg list
    assert_eq!(
        parse_sig_raw(r#"extern "C" sf::Context *sfContext_create()"#),
        RawSig {
            ret_type: "sf::Context",
            name: "*sfContext_create",
            args: vec![],
        }
    );
    assert_eq!(
        parse_sig_raw(
            r#"extern "C" sf::Transform const *sfSprite_getTransform(const sf::Sprite *sprite)"#
        ),
        RawSig {
            ret_type: "sf::Transform const",
            name: "*sfSprite_getTransform",
            args: vec![RawArg {
                name: "*sprite",
                type_: "const sf::Sprite",
            }]
        }
    );
}
