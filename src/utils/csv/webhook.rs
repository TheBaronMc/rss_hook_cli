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