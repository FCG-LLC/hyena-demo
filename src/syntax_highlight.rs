use syntect::easy::HighlightLines;
use syntect::parsing::SyntaxSet;
use syntect::highlighting::{Style, ThemeSet};
use syntect::util::as_24_bit_terminal_escaped;

pub fn highlight<S: AsRef<str>>(ty: &str, text: S) {
    // Load these once at the start of your program
    let ps = SyntaxSet::load_defaults_nonewlines();
    let ts = ThemeSet::load_defaults();

    let syntax = ps.find_syntax_by_extension(ty)
        .expect("Unable to find highlighting grammar");
    let mut theme = ts.themes["base16-eighties.dark"].clone();
    theme.settings.background.as_mut().map(|color| {
        color.r = 0x2c;
        color.g = 0x2c;
        color.b = 0x2c;
        color.a = 0xff;
    });

    let mut h = HighlightLines::new(syntax, &theme);
    for line in text.as_ref().lines() {
        let ranges: Vec<(Style, &str)> = h.highlight(line);
        let escaped = as_24_bit_terminal_escaped(&ranges[..], true);
        println!("{}", escaped);
    }

    println!("\x1b[0m");
}
