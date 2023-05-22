use crate::types::Article;
use super::CSVExport;

impl CSVExport for Article {
    fn header(&self) -> String {
        String::from("id;title;description;pub_date;url;source")
    }

    fn to_csv(&self) -> String {
        let no_description = String::from("-");
        let no_url = String::from("-");

        String::from(
            format!("{};{};{};{};{};{}",
                self.id,
                self.title,
                if let Some(description) = &self.description {
                    description
                } else {
                    &no_description
                },
                self.pub_date,
                if let Some(url) = &self.url {
                    url
                } else {
                    &no_url
                },
                self.sourceId
            )
        )
    }
}
