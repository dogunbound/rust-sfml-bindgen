use std::str::pattern::Pattern;

#[derive(Debug, PartialEq)]
pub(crate) struct RawSig<'a> {
    pub(crate) ret_type: &'a str,
    pub(crate) name: &'a str,
    pub(crate) args: Vec<RawArg<'a>>,
}

#[derive(Debug, PartialEq)]
pub(crate) struct RawArg<'a> {
    pub(crate) type_: &'a str,
    pub(crate) name: &'a str,
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
        .unwrap();
    let type_start = find_arg_delim_left_rev(&input[..name_start]);
    RawArg {
        type_: input[type_start..name_start].trim(),
        name: input[name_start..name_end].trim(),
    }
}

fn find_abs<'a, P: Pattern<'a>>(input: &'a str, start: usize, pat: P) -> usize {
    input[start..].find(pat).unwrap() + start
}

pub(crate) fn parse_sig_raw(mut input: &str) -> RawSig {
    input = input.trim_start_matches(r#"extern "C""#);
    input = input.trim_start();
    let first_space = input.find(|c: char| c.is_whitespace()).unwrap();
    let type_ = &input[..first_space];
    dbg!(type_);
    let args_open = find_abs(input, first_space + 1, '(');
    let fname = &input[first_space + 1..args_open];
    dbg!(fname);
    let args_close = find_abs(input, args_open + 1, ')');
    let args_part = &input[args_open + 1..args_close];
    dbg!(args_part);
    let args = parse_args_raw(args_part);
    RawSig {
        name: fname,
        ret_type: type_,
        args,
    }
}

fn parse_args_raw(input: &str) -> Vec<RawArg> {
    dbg!(input);
    input.split(',').map(parse_arg_raw).collect()
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
    )
}
