use std::str::pattern::{Pattern, ReverseSearcher};

pub(crate) fn find_abs<P: Pattern>(input: &str, start: usize, pat: P) -> Option<usize> {
    input[start..].find(pat).map(|idx| idx + start)
}

pub(crate) fn rfind_abs<P: Pattern>(input: &str, start: usize, pat: P) -> Option<usize>
where
    for<'a> P::Searcher<'a>: ReverseSearcher<'a>,
{
    input[..start].rfind(pat).map(|offset| offset + 1)
}

#[test]
fn test() {
    use pretty_assertions::assert_eq;
    // Case 1
    let input =
        r#"extern "C" const sfView *sfRenderWindow_getView(const sfRenderWindow *renderWindow) {"#;
    let first_paren = find_abs(input, 0, '(').unwrap();
    assert_eq!(
        &input[0..first_paren],
        r#"extern "C" const sfView *sfRenderWindow_getView"#
    );
    let first_space_back = rfind_abs(input, first_paren, |c: char| c.is_whitespace()).unwrap();
    assert_eq!(
        &input[first_space_back..first_paren],
        "*sfRenderWindow_getView"
    );
    // Case 2
    let input = "sfVector2i sfTouch_getPositionRenderWindow(unsigned int finger, const sfRenderWindow *relativeTo)";
    let args_open = find_abs(input, 0, '(').unwrap();
    assert_eq!(
        &input[0..args_open],
        "sfVector2i sfTouch_getPositionRenderWindow"
    );
    let first_space_rev = rfind_abs(input, args_open, |c: char| c.is_whitespace()).unwrap();
    assert_eq!(
        &input[first_space_rev..args_open],
        "sfTouch_getPositionRenderWindow"
    );
}
