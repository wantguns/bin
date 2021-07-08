use std::fs;

use syntect::highlighting::ThemeSet;
use syntect::html::highlighted_html_for_string;
use syntect::parsing::SyntaxSet;

pub fn get_pretty_body(path: &String, ext: &String) -> String {
    let ss = SyntaxSet::load_defaults_newlines();
    let theme = ThemeSet::get_theme("themes/ayu_dark.tmTheme").unwrap();

    let content = fs::read_to_string(path).unwrap();
    let syntax = ss
        .find_syntax_by_token(ext)
        .unwrap_or_else(|| ss.find_syntax_plain_text());
    let html = highlighted_html_for_string(&content, &ss, syntax, &theme);

    html
}
