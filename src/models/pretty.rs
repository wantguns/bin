use std::fs;
use std::path::Path;

use syntect::highlighting::ThemeSet;
use syntect::html::highlighted_html_for_string;
use syntect::parsing::SyntaxSet;

pub fn get_pretty_body(path: &Path, ext: &str) -> String {
    let ss = SyntaxSet::load_defaults_newlines();

    let mut theme_cursor =
        std::io::Cursor::new(include_bytes!("../../themes/ayu_dark.tmTheme"));
    let theme = ThemeSet::load_from_reader(&mut theme_cursor).unwrap();

    let content = fs::read_to_string(path).unwrap();
    let syntax = ss
        .find_syntax_by_token(ext)
        .unwrap_or_else(|| ss.find_syntax_plain_text());

    highlighted_html_for_string(&content, &ss, syntax, &theme)
}
