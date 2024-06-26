#[macro_use]
extern crate gfx;
extern crate gfx_text;
extern crate gfx_window_glutin;
extern crate glutin;

pub mod commands;
pub mod render;
pub mod dom;
pub mod html_parser;
pub mod css_parser;
pub mod css;
pub mod style;
pub mod layout;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
