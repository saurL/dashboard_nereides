use linux_embedded_hal::i2cdev::core::I2CDevice;
use linux_embedded_hal::I2cdev;
use std::thread::sleep;
use std::{thread, time::Duration};
use std::str;
use tauri::async_runtime::{spawn, Sender};
use log::{error, info};

use crate::uart_communication::{UartData, UartDataNumber};

#[derive(Clone)]
pub struct Gps{
    tx: Sender<UartData>,
}

impl Gps {
    pub fn new(tx: Sender<UartData>) -> Self {
        let instance = Gps {tx};
        instance.start_reading();
        instance
    }

    pub fn start_reading(&self){
        let tx: Sender<UartData>= self.tx.clone();

        spawn(async move {
            info!("GPS: Initialisation de la lecture des données GPS");
            // Initialiser le périphérique I2C
            let mut i2c: I2cdev = I2cdev::new("/dev/i2c-2").expect("Failed to open I2C device");
            info!("GPS: Périphérique I2C ouvert avec succès");
            i2c.set_slave_address(0x42).expect("Failed to set slave address");
            let mut buffer = String::new();
            loop{
            info!("GPS: Initialisation de la lecture des données GPS");
                
            let data =match i2c.smbus_read_block_data(0xFF) {
                Ok(data) => {
                    info!("GPS: lecture d'I2C réussie");
                    data // Stocker le retour dans une variable
                }
                Err(e) => {
                    error!("Erreur de lecture d'I2C: {:?}", e);
                    continue;
                }
            };
                
              // Petite pause pour laisser le périphérique se préparer après l'écriture


                info!("GPS: Données lues: {:?}", data);
                // Convertir les octets en texte ASCII lisible
                let texte: String = data
                    .iter()
                    .filter_map(|&b| if (32..127).contains(&b) { Some(b as char) } else { None })
                    .collect();
                info!("GPS: Données converties: {:?}", texte);
                buffer.push_str(&texte);
                info!("GPS: Buffer après ajout: {:?}", buffer);
                // Traitement des trames GPS
                let mut latitude: Option<f64> = None;
                let mut longitude: Option<f64> = None;
                let mut vitesse_kmh: Option<f64> = None;
        
                // GNGGA: position
                if let Some(start) = buffer.find("$GNGGA") {
                    if let Some(end) = buffer[start..].find('\n') {
                        let line = buffer[start..start + end].to_string();
                        buffer.replace_range(start..start + end + 1, "");
                        let champs: Vec<&str> = line.split(',').map(str::trim).collect();
        
                        if champs.len() > 5 && !champs[2].is_empty() && !champs[4].is_empty() {
                            let raw_lat = champs[2];
                            let lat_dir = champs[3];
                            let deg_lat = raw_lat[..2].parse::<f64>().unwrap_or(0.0);
                            let min_lat = raw_lat[2..].parse::<f64>().unwrap_or(0.0);
                            let mut lat = deg_lat + (min_lat / 60.0);
                            if lat_dir == "S" {
                                lat *= -1.0;
                            }
        
                            let raw_lon = champs[4];
                            let lon_dir = champs[5];
                            let deg_lon = raw_lon[..3].parse::<f64>().unwrap_or(0.0);
                            let min_lon = raw_lon[3..].parse::<f64>().unwrap_or(0.0);
                            let mut lon = deg_lon + (min_lon / 60.0);
                            if lon_dir == "W" {
                                lon *= -1.0;
                            }
        
                            latitude = Some(lat);
                            longitude = Some(lon);
                            info!("Latitude: {:.6}°, Longitude: {:.6}°", lat, lon);
                        }
                    }
                }
        
                // GNRMC: vitesse
                if let Some(start) = buffer.find("$GNRMC") {
                    if let Some(end) = buffer[start..].find('\n') {
                        let line = buffer[start..start + end].to_string();
                        buffer.replace_range(start..start + end + 1, "");
                        let champs: Vec<&str> = line.split(',').map(str::trim).collect();
        
                        if champs.len() > 7 && !champs[7].is_empty() {
                            if let Ok(speed_knots) = champs[7].parse::<f64>() {
                                vitesse_kmh = Some(speed_knots * 1.852);
                                info!("Vitesse: {:.2} km/h", vitesse_kmh.unwrap());
                            }
                        }
                    }
                }
                if let Some(lat) = latitude{
                    let data: UartDataNumber = UartDataNumber{
                        data_name: "gps_latitude".to_string(),
                        value: lat,
                    };
                    tx.send(UartData::Number(data)).await.unwrap();
                }
        
               if let Some(lon) = longitude{
                let data: UartDataNumber = UartDataNumber{
                    data_name: "gps_longitude".to_string(),
                    value: lon,
                };
                tx.send(UartData::Number(data)).await.unwrap();
                }
                if let Some(vitesse) = vitesse_kmh{
                    let data: UartDataNumber = UartDataNumber{
                        data_name: "gps_vitesse".to_string(),
                        value: vitesse,
                    };
                    tx.send(UartData::Number(data)).await.unwrap();
                }
        
                thread::sleep(Duration::from_millis(200));
            
            }
        });
    }
}
    
    
