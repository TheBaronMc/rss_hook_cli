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