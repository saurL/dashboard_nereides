use super::structs::Line;
use csv::ReaderBuilder;
use std::error::Error;
use std::fs::File;
use std::path::Path;

pub fn read_csv(file_path: &str) -> Result<Vec<Line>, Box<dyn Error + Send>> {
    let chemin = Path::new(file_path);
    let fichier: File = if !chemin.exists() {
        File::create(file_path).map_err(|e| Box::new(e) as Box<dyn Error + Send>)?
    } else {
        File::open(chemin).map_err(|e| Box::new(e) as Box<dyn Error + Send>)?
    };

    // Write headers if the file was just created

    // n'arrive pas a écrire dans un fichier
    if fichier
        .metadata()
        .map_err(|e| Box::new(e) as Box<dyn Error + Send>)?
        .len()
        == 0
    {
        let mut wtr = csv::Writer::from_writer(&fichier);

        wtr.write_record(&["timestamp", "value"])
            .map_err(|e| Box::new(e) as Box<dyn Error + Send>)?;

        // ici flush ne fonctionne pas
        wtr.flush()
            .map_err(|e| Box::new(e) as Box<dyn Error + Send>)?;
    }

    let mut lecteur: csv::Reader<File> =
        ReaderBuilder::new().has_headers(true).from_reader(fichier);

    let mut lignes: Vec<Line> = Vec::new();
    for result in lecteur.deserialize() {
        let ligne: Line = result.map_err(|e| Box::new(e) as Box<dyn Error + Send>)?;

        lignes.push(ligne);
    }

    /*for ligne in &lignes {
        println!("{:?}", ligne);
    } */
    Ok(lignes)
}
