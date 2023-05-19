use super::*;
use crate::types::Article;
use crate::utils::count_digits;

pub struct ArticleFormatter<'a> {
    formatter_pref: FormatterPref<'a>,
    current_max_url_length: u64,
    current_max_title_length: u64,
    max_source_id_length: u64,
    max_article_id_length: u64,
    articles: Box<Vec<Article>>
}

impl<'a> ArticleFormatter<'a> {
    pub fn new(element: Box<Vec<Article>>, pref: FormatterPref<'a>) -> Self {
        let mut formatter = ArticleFormatter {
            formatter_pref: pref,
            current_max_url_length: 4,
            current_max_title_length: 6,
            max_article_id_length: 3,
            max_source_id_length: 8,
            articles: element
        };

        for article in formatter.articles.iter() {
            let id_digits = count_digits(article.id);
            let url_len = if let Some(url) = &article.url { url.len() as u64 } else { 0 };
            let title_len = article.title.len() as  u64;
            let source_id_digits = count_digits(article.sourceId);

            if id_digits > formatter.max_article_id_length {
                formatter.max_article_id_length = id_digits;
            }
            if source_id_digits > formatter.max_source_id_length {
                formatter.max_source_id_length = source_id_digits;
            }
            if url_len > formatter.current_max_url_length {
                if url_len <= pref.max_str_len {
                    formatter.current_max_url_length = url_len;
                } else {
                    formatter.current_max_title_length = pref.max_str_len;
                }
            }
            if title_len > formatter.current_max_title_length {
                if url_len <= pref.max_str_len {
                    formatter.current_max_title_length = title_len;
                } else {
                    formatter.current_max_title_length = pref.max_str_len;
                }
            }
        }

        formatter
    }
}

impl Formatter<Vec<Article>> for ArticleFormatter<'_> {
    fn show_header(&self) {
        print_separation(self);
        println!("{} {: <width_id$} {} {: <width_title$} {} {: <width_url$} {} {:width_source$} {}", 
            self.formatter_pref.column_sep, 
            "ID",  self.formatter_pref.column_sep,
            "TITLE",  self.formatter_pref.column_sep,
            "URL",  self.formatter_pref.column_sep,
            "SOURCE",  self.formatter_pref.column_sep,
            width_id = (self.max_article_id_length + 1) as usize,
            width_title = (self.current_max_title_length + 1) as usize,
            width_url = (self.current_max_url_length + 1) as usize,
            width_source = (self.max_source_id_length + 1) as usize
        );
        print_separation(self);
    }

    fn show_content(&self) {
        for article in self.articles.iter() {
            let url = if let Some(url) = &article.url {
                if url.len() > self.formatter_pref.max_str_len as usize {
                    String::from(&url[..47]) + "..."
                } else {
                    url.clone()
                }
            } else {
                String::from("-")
            };
            let title = if article.title.len() > self.formatter_pref.max_str_len as usize {
                String::from(&article.title[..47]) + "..."
            } else {
                article.title.clone()
            };

            println!("{} {: <width_id$}{} {: <width_title$}{} {: <width_url$}{} {:width_source$}{}", 
                self.formatter_pref.column_sep, 
                article.id,  self.formatter_pref.column_sep,
                title,  self.formatter_pref.column_sep,
                url,  self.formatter_pref.column_sep,
                article.sourceId,  self.formatter_pref.column_sep,
                width_id = self.max_article_id_length as usize,
                width_title = self.current_max_title_length as usize,
                width_url = self.current_max_url_length as usize,
                width_source = self.max_source_id_length as usize
            );
        }
    }

    fn show_footer(&self) {
        print_separation(self);
    }
}

fn print_separation(formatter: &ArticleFormatter) {
    println!("{}", formatter.formatter_pref.section_sep.repeat(
        (2 + formatter.max_article_id_length + 1
            + 2 + formatter.current_max_title_length + 1
            + 2 + formatter.current_max_url_length + 1
            + 2 + formatter.max_source_id_length + 2) as usize
    ));
}