pub mod article;
pub mod webhook;
pub mod flux;

pub trait CSVExport {
    fn header(&self) -> String;
    fn to_csv(&self) -> String;
}