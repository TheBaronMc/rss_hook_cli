use crate::types::Exception;
use super::csv::CSVExport;

use std::fs::File;
use std::io::{LineWriter, Write};

pub fn write(filename: &String, exporter: &dyn CSVExport) -> Result<String, Exception> {
    let file = create_file(filename)?;

    let mut res;
    let mut writer = LineWriter::new(file);
    res = writer.write_all(exporter.header().as_bytes());
    if let Err(error) = res {
        return Err(Exception {
            statusCode: 500,
            message: error.to_string()
        });
    }
    res = writer.write_all(exporter.to_csv().as_bytes());
    if let Err(error) = res {
        return Err(Exception {
            statusCode: 500,
            message: error.to_string()
        });
    }

    Ok(filename.clone())
}

pub fn write_all(filename: &String, exporters: Vec<Box<dyn CSVExport>>) -> Result<String, Exception> {
    let file = create_file(filename)?;
    let mut writer = LineWriter::new(file);

    for exporter in exporters {
        let mut res;
        res = writer.write_all(exporter.header().as_bytes());
        if let Err(error) = res {
            return Err(Exception {
                statusCode: 500,
                message: error.to_string()
            });
        }
        res = writer.write_all(exporter.to_csv().as_bytes());
        if let Err(error) = res {
            return Err(Exception {
                statusCode: 500,
                message: error.to_string()
            });
        }
    }

    Ok(filename.clone())
}

fn create_file(filename: &String) -> Result<File, Exception> {
    let res = File::create(filename);
    match res {
        Ok(file) => Ok(file),
        Err(err) => Err( Exception { 
                statusCode: 500, 
                message: err.to_string()
            })
    }
}