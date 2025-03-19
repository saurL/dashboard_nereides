use crate::constant::{CSV_DIR_PATH, SCV_FILE_NAME};
use chrono::Local;
use csv::Writer;
use indexmap::IndexMap;
use log::error;
use std::error::Error;
use std::fs::OpenOptions;
#[derive(Clone)]
pub struct Csv_writter {
    file_path: String,
}

impl Csv_writter {
    pub fn new(data: IndexMap<&'static str, Option<f64>>) -> Self {
        std::fs::create_dir_all(CSV_DIR_PATH).unwrap_or_else(|err| {
            error!("Failed to create directory {}: {}", CSV_DIR_PATH, err);
        });
        let file_path: String = format!("{}{}.csv", CSV_DIR_PATH, SCV_FILE_NAME);
        match OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open(file_path.clone())
        {
            Ok(file) => {
                let mut wtr = Writer::from_writer(file);
                let mut record = vec!["timestamp"];
                for data_name in data.keys() {
                    record.push(data_name);
                }
                wtr.write_record(record)
                    .map_err(|e| {
                        error!("Failed to write record: {}", e);
                        Box::<dyn Error>::from("Failed to write record")
                    })
                    .unwrap();
            }
            Err(err) => error!("Failed to open file: {}", err),
        }

        Self { file_path }
    }

    pub fn write_data(&mut self, data: IndexMap<&'static str, f64>) -> Result<(), Box<dyn Error>> {
        let file_path: String = self.file_path.clone();
        match OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open(file_path)
        {
            Ok(file) => {
                let mut wtr = Writer::from_writer(file);
                let date_time = Local::now().format("%Y-%m-%d %H-%M-%S").to_string();
                let mut record = vec![date_time];
                for (data_name, value) in data {
                    record.push(value.to_string());
                }
                wtr.write_record(record).map_err(|e| {
                    error!("Failed to write record: {}", e);
                    Box::<dyn Error>::from("Failed to write record")
                })?;
            }
            Err(err) => error!("Failed to open file: {}", err),
        }

        Ok(())
    }
}
