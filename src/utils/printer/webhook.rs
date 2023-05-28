use super::*;
use crate::types::Webhook;
use crate::utils::count_digits;

pub struct WebhookFormatter<'a> {
    formatter_pref: FormatterPref<'a>,
    current_max_url_length: u64,
    max_id_length: u64,
    webhooks: Box<Vec<Webhook>>
}

impl<'a> WebhookFormatter<'a> {
    pub fn new(element: Box<Vec<Webhook>>, pref: FormatterPref<'a>) -> Self {
        let mut formatter = WebhookFormatter {
            formatter_pref: pref,
            current_max_url_length: 4,
            max_id_length: 3,
            webhooks: element
        };

        for webhook in formatter.webhooks.iter() {
            let id_digits = count_digits(webhook.id);
            let url_len = webhook.url.len() as u64;

            if id_digits > formatter.max_id_length {
                formatter.max_id_length = id_digits;
            }
            if url_len > formatter.current_max_url_length {
                if url_len <= pref.max_str_len {
                    formatter.current_max_url_length = url_len;
                } else {
                    formatter.current_max_url_length = pref.max_str_len;
                }
            }
        }

        formatter
    }
}

impl Formatter<Vec<Webhook>> for WebhookFormatter<'_> {
    fn show_header(&self) {
        print_separation(self);
        println!("{} {: <width_id$} {} {: <width_url$} {}", 
            self.formatter_pref.column_sep, 
            "ID",  self.formatter_pref.column_sep,
            "URL",  self.formatter_pref.column_sep,
            width_id = self.max_id_length as usize,
            width_url = self.current_max_url_length as usize,
        );
        print_separation(self);
    }

    fn show_content(&self) {
        for webhook in self.webhooks.iter() {
            let url = if webhook.url.len() > self.formatter_pref.max_str_len as usize {
                String::from(&webhook.url[..47]) + "..."
            } else {
                webhook.url.clone()
            };

            println!("{} {: <width_id$} {} {: <width_url$} {}", 
                self.formatter_pref.column_sep, 
                webhook.id,  self.formatter_pref.column_sep,
                url,  self.formatter_pref.column_sep,
                width_id = self.max_id_length as usize,
                width_url = self.current_max_url_length as usize,
            );
        }
    }

    fn show_footer(&self) {
        print_separation(self);
    }
}

fn print_separation(formatter: &WebhookFormatter) {
    println!("{}", formatter.formatter_pref.section_sep.repeat(
        (2 + formatter.max_id_length + 1
            + 2 + formatter.current_max_url_length + 2) as usize
    ));
}