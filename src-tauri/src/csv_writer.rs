use crate::constant::{CSV_DIR_PATH, DATAS_NAMES};
use chrono::Local;
use csv::Writer;
use log::error;
use std::error::{self, Error};
use std::fs::OpenOptions;

#[derive(Clone)]
pub struct Csv_writter {}

impl Csv_writter {
    pub fn new() -> Self {
        std::fs::create_dir_all(CSV_DIR_PATH).unwrap_or_else(|err| {
            error!("Failed to create directory {}: {}", CSV_DIR_PATH, err);
        });
        for datanames in DATAS_NAMES.iter() {
            let file_path: String = format!("{}{}.csv", CSV_DIR_PATH, datanames);
            match OpenOptions::new()
                .write(true)
                .append(true)
                .create(true)
                .open(file_path)
            {
                Ok(file) => {
                    let mut wtr = Writer::from_writer(file);
                    wtr.write_record(&["timestamp", "value"])
                        .map_err(|e| {
                            error!("Failed to write record: {}", e);
                            Box::<dyn Error>::from("Failed to write record")
                        })
                        .unwrap();
                }
                Err(err) => error!("Failed to open file: {}", err),
            }
        }
        Self {}
    }

    pub fn write_data(&mut self, data_name: &str, value: f64) -> Result<(), Box<dyn Error>> {
        let file_path: String = format!("{}{}.csv", CSV_DIR_PATH, data_name);
        match OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open(file_path)
        {
            Ok(file) => {
                let mut wtr = Writer::from_writer(file);
                let date_time = Local::now().format("%Y-%m-%d %H-%M-%S").to_string();

                wtr.write_record(&[&date_time, &value.to_string()])
                    .map_err(|e| {
                        error!("Failed to write record: {}", e);
                        Box::<dyn Error>::from("Failed to write record")
                    })?;
            }
            Err(err) => error!("Failed to open file: {}", err),
        }

        Ok(())
    }
}
