use log::{error, info};
use serde::{de, Deserialize, Serialize};

use serialport::SerialPort;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::mpsc::Sender;
use tokio::sync::Mutex;
#[derive(Clone)]
pub struct UartCommunication {
    port: Arc<Mutex<Box<dyn SerialPort>>>,
    tx: Sender<UartData>,
}
#[derive(Deserialize, Serialize)]
pub struct UartDataNumber {
    pub data_name: String,
    pub value: f64,
}
#[derive(Deserialize, Serialize)]

pub struct UartDataString {
    pub data_name: String,
    pub value: String,
}
#[derive( Deserialize)]
#[serde(untagged)]
pub enum UartData {
    Number(UartDataNumber),
    String(UartDataString),
}

impl UartCommunication {
    pub fn new(port_name: &str, baud_rate: u32, tx: Sender<UartData>) -> Self {
        let port: Box<dyn SerialPort> = serialport::new(port_name, baud_rate)
            .timeout(Duration::from_millis(10))
            .open()
            .expect("Failed to open port");

        let instance = UartCommunication {
            port: Arc::new(port.into()),
            tx,
        };
        instance.start_reading();
        instance
    }

    pub fn start_reading(&self) -> tokio::task::JoinHandle<()> {
        let port_clone = self.port.clone();
        let tx: Sender<UartData> = self.tx.clone();
        tokio::spawn(async move {
            let mut buffer = vec![0u8; 1024];
            loop {
                let mut port = port_clone.lock().await;
                match port.read(&mut buffer) {
                    Ok(bytes_read) if bytes_read > 0 => match decode_varint(&buffer) {
                        Ok(some) => match some {
                            Some((size, bytes_read)) => {
                                if size as usize > buffer.len() - bytes_read {
                                    error!("Invalid data length: {}", size);
                                    continue;
                                }
                                let data = &buffer[bytes_read..bytes_read + size as usize];
                                let data_str = match std::str::from_utf8(data) {
                                    Ok(s) => s,
                                    Err(e) => {
                                        error!("Failed to convert data to string: {}", e);
                                        continue;
                                    }
                                };
                                let json_value:UartData  =
                                    match serde_json::from_str(data_str) {
                                        Ok(json) => json,
                                        Err(e) => {
                                            error!("Failed to parse JSON: {}", e);
                                            continue;
                                        }
                                    };
                                info!("Received data: {}", data_str);
                                if tx.send(json_value).await.is_err() {
                                    error!("Failed to send data to channel");
                                }
                                buffer = buffer[..size as usize].to_vec();
                            }
                            None => continue,
                        },
                        Err(e) => {
                            error!("Failed to decode varint: {}", e);
                        }
                    },
                    Ok(_) => continue,
                    Err(e) => {
                        error!("Error reading from UART: {}", e);
                    }
                }
            }
        })
    }
}

fn decode_varint(buffer: &[u8]) -> Result<Option<(u64, usize)>, std::io::Error> {
    let mut value = 0u64;
    let mut shift = 0;
    let mut bytes_read = 0;

    for byte in buffer.iter() {
        bytes_read += 1;

        // Applique un masque 0x7F pour obtenir les 7 bits significatifs
        value |= (*byte as u64 & 0x7F) << shift;

        // Si le bit de continuation (0x80) est désactivé, la lecture est terminée
        if *byte & 0x80 == 0 {
            return Ok(Some((value, bytes_read)));
        }

        // Incrémente le décalage (shift) pour le prochain groupe de 7 bits
        shift += 7;

        // Si le décalage atteint 64 bits, cela signifie que la Varint est trop longue
        if shift >= 64 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Varint is too long",
            ));
        }
    }
    Ok(None) // Si la lecture est incomplète, retourne None
}
