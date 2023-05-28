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

#[cfg(test)]
mod tests {
    use crate::types::Article;
    use crate::utils::csv::CSVExport;

    #[test]
    fn header_test() {
        let article = Article {
            id: 1,
            title: String::from("test"),
            description: None,
            pub_date: String::from("today"),
            url: None,
            sourceId: 1
        };

        assert_eq!(article.header(), "id;title;description;pub_date;url;source");
    }

    #[test]
    fn content_test() {
        let partial_article = Article {
            id: 1,
            title: String::from("test"),
            description: None,
            pub_date: String::from("today"),
            url: None,
            sourceId: 1
        };

        let complete_article = Article {
            id: 1,
            title: String::from("test"),
            description: Some(String::from("Description")),
            pub_date: String::from("today"),
            url: Some(String::from("http://some.url")),
            sourceId: 1
        };

        assert_eq!(
            partial_article.to_csv(),
            "1;test;-;today;-;1"
        );

        assert_eq!(
            complete_article.to_csv(),
            "1;test;Description;today;http://some.url;1"
        );
    }
}
