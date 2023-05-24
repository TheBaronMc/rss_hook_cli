use crate::types::Flux;
use super::CSVExport;

impl CSVExport for Flux {
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
    use crate::types::Flux;
    use crate::utils::csv::CSVExport;

    #[test]
    fn header_test() {
        let flux = Flux {
            id: 1,
            url: String::from("http://some.url")
        };

        assert_eq!(flux.header(), "id;url");
    }

    #[test]
    fn content_test() {
        let flux = Flux {
            id: 1,
            url: String::from("http://some.url")
        };

        assert_eq!(flux.to_csv(), "1;http://some.url");
    }
}