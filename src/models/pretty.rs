use std::fs;
use std::path::Path;

use syntect::highlighting::ThemeSet;
use syntect::html::highlighted_html_for_string;
use syntect::parsing::SyntaxSet;

static SYNTAXES: &[u8] =
    include_bytes!("../../resources/syntaxes/syntaxes.bin");
static THEMES: &[u8] =
    include_bytes!("../../resources/themes/ayu_dark.tmTheme");

pub fn get_pretty_body(path: &Path, ext: &str) -> std::io::Result<String> {
    let ss: SyntaxSet = syntect::dumps::from_binary(SYNTAXES);

    let mut theme_cursor = std::io::Cursor::new(THEMES);
    let theme = ThemeSet::load_from_reader(&mut theme_cursor).unwrap();

    let content = fs::read_to_string(path)?;
    let syntax = ss
        .find_syntax_by_token(ext)
        .unwrap_or_else(|| ss.find_syntax_plain_text());

    Ok(highlighted_html_for_string(&content, &ss, syntax, &theme))
}
