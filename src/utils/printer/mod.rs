
pub mod article;
pub mod webhook;
pub mod flux;

#[derive(Copy, Clone)]
pub struct FormatterPref<'a> {
    pub max_str_len: u64,
    pub column_sep: &'a str,
    pub section_sep: &'a str
}
pub trait Formatter<T> {
    fn show_header(&self);
    fn show_content(&self);
    fn show_footer(&self);
}

pub fn print<T>(printer: Box<dyn Formatter<T>>) {
    printer.show_header();
    printer.show_content();
    printer.show_footer();
}