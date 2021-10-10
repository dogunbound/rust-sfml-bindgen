#![feature(pattern)]

//! Tool that extracts signatures from a bunch of C(++) functions
//! and converts them to Rust form

mod parse;

fn main() {
    dbg!(parse::parse(
        r#"extern "C" void sfRenderWindow_drawPrimitives(sfRenderWindow *renderWindow,
        const sfVertex *vertices, size_t vertexCount,
        sfPrimitiveType type, const sf::RenderStates *states) "#
    ));
}
