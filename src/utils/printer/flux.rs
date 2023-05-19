use super::*;
use crate::types::Flux;
use crate::utils::count_digits;

pub struct FluxFormatter<'a> {
    formatter_pref: FormatterPref<'a>,
    current_max_url_length: u64,
    max_id_length: u64,
    flux: Box<Vec<Flux>>
}

impl<'a> FluxFormatter<'a> {
    pub fn new(element: Box<Vec<Flux>>, pref: FormatterPref<'a>) -> Self {
        let mut formatter = FluxFormatter {
            formatter_pref: pref,
            current_max_url_length: 4,
            max_id_length: 3,
            flux: element
        };

        for flux in formatter.flux.iter() {
            let id_digits = count_digits(flux.id);
            let url_len = flux.url.len() as u64;

            if id_digits > formatter.max_id_length {
                formatter.max_id_length = id_digits;
            }
            if url_len > formatter.current_max_url_length && url_len <= pref.max_str_len {
                formatter.current_max_url_length = url_len;
            }
        }

        formatter
    }
}

impl Formatter<Vec<Flux>> for FluxFormatter<'_> {
    fn show_header(&self) {
        print_separation(self);
        println!("{} {: <width_id$}{} {: <width_url$}{}", 
            self.formatter_pref.column_sep, 
            "ID",  self.formatter_pref.column_sep,
            "URL",  self.formatter_pref.column_sep,
            width_id = (self.max_id_length + 1) as usize,
            width_url = (self.current_max_url_length + 1) as usize,
        );
        print_separation(self);
    }

    fn show_content(&self) {
        for flux in self.flux.iter() {
            println!("{} {: <width_id$} {} {: <width_url$} {}", 
                self.formatter_pref.column_sep, 
                flux.id,  self.formatter_pref.column_sep,
                flux.url,  self.formatter_pref.column_sep,
                width_id = self.max_id_length as usize,
                width_url = self.current_max_url_length as usize,
            );
        }
    }

    fn show_footer(&self) {
        print_separation(self);
    }
}

fn print_separation(formatter: &FluxFormatter) {
    println!("{}", formatter.formatter_pref.section_sep.repeat(
        (2 + formatter.max_id_length + 1
            + 2 + formatter.current_max_url_length + 2) as usize
    ));
}