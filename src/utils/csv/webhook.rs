use crate::types::Webhook;
use super::CSVExport;

impl CSVExport for Webhook {
    fn header(&self) -> String {
        String::from("id;url")
    }

    fn to_csv(&self) -> String {
        String::from(
            format!("{};{}",
                self.id,
                self.url
            )
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::types::Webhook;
    use crate::utils::csv::CSVExport;

    #[test]
    fn header_test() {
        let webhook = Webhook {
            id: 1,
            url: String::from("http://some.url")
        };

        assert_eq!(webhook.header(), "id;url");
    }

    #[test]
    fn content_test() {
        let webhook = Webhook {
            id: 1,
            url: String::from("http://some.url")
        };

        assert_eq!(webhook.to_csv(), "1;http://some.url");
    }
}