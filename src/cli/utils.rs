use colored::Colorize;

pub fn aly(letter: &str, show_logo: bool) -> String {
    let mut wrapped = "[ ".bright_green().to_string();
    wrapped.push_str(&letter.bright_green().bold().to_string());
    wrapped.push_str(&" ]".bright_green().to_string());
    wrapped.push_str(&"  ".to_string());
    let new_wrapped = if show_logo { wrapped } else { "".to_string() };
    new_wrapped
}