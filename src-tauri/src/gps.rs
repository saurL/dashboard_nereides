use linux_embedded_hal::i2cdev::core::I2CDevice;
use linux_embedded_hal::I2cdev;
use openssl::derive;
use std::{thread, time::Duration};
use std::str;
use tauri::async_runtime::spawn;

#[derive(Clone)]
pub struct Gps{

}

impl Gps {
    pub fn new() -> Self {
        let instance = Gps {};
        instance.start_reading();
        instance
    }

    pub fn start_reading(&self){
        spawn(async move {
            let mut i2c: I2cdev = I2cdev::new("/dev/i2c-1").expect("Failed to open I2C device");

            let mut buffer = String::new();
           
            loop {
                let mut data: [u8; 32] = [0u8; 32];
                if let Err(e) = i2c.read(&mut data) {
                    eprintln!("Erreur de lecture I2C: {:?}", e);
                    continue;
                }
        
                // Convertir les octets en texte ASCII lisible
                let texte: String = data
                    .iter()
                    .filter_map(|&b| if (32..127).contains(&b) { Some(b as char) } else { None })
                    .collect();
        
                buffer.push_str(&texte);
        
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
                            }
                        }
                    }
                }
        
                if let (Some(lat), Some(lon), Some(vit)) = (latitude, longitude, vitesse_kmh) {
                    println!("Latitude  : {:.6}°", lat);
                    println!("Longitude : {:.6}°", lon);
                    println!("Vitesse   : {:.2} km/h", vit);
                    println!("-----------------------------");
                }
        
                thread::sleep(Duration::from_millis(200));
            
            }
        });
    }
}
    
    
