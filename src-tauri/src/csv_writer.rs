use csv::Writer;
use std::error::Error;
use std::fs::OpenOptions;
use std::io::Write;
pub struct Csv_writter {}
fn main() -> Result<(), Box<dyn Error>> {
    let file_path = "data.csv";

    // Ouvre le fichier en mode append (ajout) ou le crée s'il n'existe pas
    let file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(file_path)?;

    let mut wtr = Writer::from_writer(file);

    // Écriture des données (exemple)
    let data = vec![
        ("2025-03-08", "temperature", 22.5),
        ("2025-03-08", "humidity", 60.0),
    ];
    /*
    for row in data {
        wtr.write_record(&[row.0, row.1, row.2.to_string()])?;
    } */

    wtr.flush()?; // Assure l'écriture sur le disque
    println!("Données ajoutées avec succès dans {}", file_path);

    Ok(())
}
