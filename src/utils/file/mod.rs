use crate::types::Exception;
use super::csv::CSVExport;

use std::fs::File;
use std::io::{LineWriter, Write};

pub fn write(filename: &String, exporter: &dyn CSVExport) -> Result<String, Exception> {
    let file = create_file(filename)?;

    let mut writer = LineWriter::new(file);
    write_line(&mut writer, &(exporter.header() + "\n"))?;
    write_line(&mut writer, &exporter.to_csv())?;

    Ok(filename.clone())
}

pub fn write_all(filename: &String, exporters: &Vec<Box<dyn CSVExport>>) -> Result<String, Exception> {
    let file = create_file(filename)?;
    let mut writer = LineWriter::new(file);

    for i in 0..exporters.len() {
        if i == 0 {
            write_line(&mut writer, &(exporters[i].header() + "\n"))?;
        }

        let content = exporters[i].to_csv() + if i < exporters.len()-1 { "\n" } else { "" };
        write_line(&mut writer, &content)?;
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

fn write_line(writer: &mut LineWriter<File>, line: &String) -> Result<(), Exception>{
    if let Err(error) = writer.write_all(line.as_bytes()) {
        return Err(Exception {
            statusCode: 500,
            message: error.to_string()
        });
    }

    Ok(())
} 


#[cfg(test)]
mod tests {
    use crate::{utils::csv::CSVExport, types::Exception};
    use super::{write, write_all};

    use std::fs;

    struct Object {}

    const HEADER: &str = "field1;field2";
    const CONTENT: &str = "test;test";
    const TMP: &str = "tmp";

    impl CSVExport for Object {
        fn header(&self) -> String {
            String::from(HEADER)
        }

        fn to_csv(&self) -> String {
            String::from(CONTENT)
        }
    }

    #[test]
    #[allow(unused_must_use)]
    fn write_test() -> Result<(), Exception> {
        fs::remove_file(TMP);

        let obj = Object {};

        let filename = String::from(TMP);

        write(&filename, &obj)?;

        let contents = fs::read_to_string(filename)
            .expect("No file ");

        assert_eq!(contents, format!("{}\n{}", HEADER, CONTENT));

        fs::remove_file(TMP);

        Ok(())
    }

    #[test]
    #[allow(unused_must_use)]
    fn write_all_test() -> Result<(), Exception> {
        #[warn(unused_results)]
        fs::remove_file(TMP);

        let mut objects: Vec<Box<dyn CSVExport>> = Vec::new();
        for _ in 0..3 {
            objects.push(Box::new(Object {}));
        }

        let filename = String::from(TMP);

        write_all(&filename, &objects)?;

        let contents = fs::read_to_string(filename)
            .expect("No file ");

        assert_eq!(contents, format!("{}\n{}\n{}\n{}", HEADER, CONTENT, CONTENT, CONTENT));

        fs::remove_file(TMP);

        Ok(())
    }

}