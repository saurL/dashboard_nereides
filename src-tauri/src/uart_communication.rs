use log::{error, info};
use serde::{de, Deserialize, Serialize};

use serialport::SerialPort;
use std::sync::Arc;
use std::time::Duration;
use tauri::async_runtime::{spawn, JoinHandle};
use tokio::sync::mpsc::Sender;
use tokio::sync::Mutex;
#[derive(Clone)]
pub struct UartCommunication {
    port: Arc<Mutex<Box<dyn SerialPort>>>,
    tx: Sender<UartData>,
    good_packets: Arc<Mutex<u64>>,
    bad_packets: Arc<Mutex<u64>>,
    percentage: Arc<Mutex<f64>>,
}
#[derive(Deserialize, Serialize, Debug)]
pub struct UartDataNumber {
    pub data_name: String,
    pub value: f64,
}
#[derive(Deserialize, Serialize, Debug)]

pub struct UartDataString {
    pub data_name: String,
    pub value: String,
}
#[derive(Deserialize, Debug)]
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
            good_packets: Arc::new(Mutex::new(0)),
            bad_packets: Arc::new(Mutex::new(0)),
            percentage: Arc::new(Mutex::new(0.0)),
        };
        instance.start_reading();
        instance
    }

    pub fn start_reading(&self) -> JoinHandle<()> {
        let instance = self.clone();

        spawn(async move {
            let mut total_buffer = Vec::new();
            loop {
                let mut buffer = vec![0u8; 1024];

                match instance.port.lock().await.read(&mut buffer) {
                    Ok(bytes_read) if bytes_read > 0 => {
                        if buffer.iter().all(|&x| x == 0) {
                            continue;
                        }
                        // Retirer tous les 0 du buffer
                        buffer = buffer[..bytes_read].into();

                        total_buffer.extend(buffer);

                        info!("Total buffer state: {:?}", total_buffer);

                        // Si jamais le deuxème élément du buffer n'est pas '{', on fait en sorte de chercher le prochain
                        if total_buffer.len() > 1 && total_buffer[1] != b'{' {
                            if let Some(pos) = total_buffer.iter().skip(1).position(|&x| x == b'{')
                            {
                                total_buffer.drain(..pos); // Supprime tout jusqu'au premier '{'
                                info!("Buffer after removing invalid data: {:?}", total_buffer);
                            } else {
                                error!("Invalid data format, no '{{' found");
                                total_buffer.clear(); // ou `return` si tu veux quitter complètement
                                continue;
                            }
                        }
                        match std::str::from_utf8(&total_buffer) {
                            Ok(s) => info!("Total buffer as string: {:?}", s),
                            Err(e) => {}
                        };
                        while let Ok(Some((size, bytes_read))) = decode_varint(&total_buffer) {
                            if size as usize > total_buffer.len() - bytes_read {
                                error!("Invalid data length: {}", size);
                                break; // ou `return` si tu veux quitter complètement
                            }

                            let data = &total_buffer[bytes_read..bytes_read + size as usize];

                            let data_str = match std::str::from_utf8(data) {
                                Ok(s) => s,
                                Err(e) => {
                                    error!("Failed to convert data to string: {} Cette erreur n'est absolument pas normale , des données sont perdues et pour que la communication perdure \nNous allons tronquer le buffer jusqu'a obtenir le prochain début de message", e);
                                    //S'il y a des caratères non valide, on tronque le buffer jusqu'au prochain début de message
                                    // On cherche le prochain début de message

                                    total_buffer.clear();
                                    let mut bad_packed = instance.bad_packets.lock().await;
                                    *bad_packed += 1;
                                    continue;
                                }
                            };

                            info!("Data string: {}", data_str);

                            let json_value: UartData = match serde_json::from_str(data_str) {
                                Ok(json) => json,
                                Err(e) => {
                                    error!("Failed to parse JSON: {}", e);
                                    total_buffer =
                                        total_buffer[bytes_read + size as usize..].to_vec();
                                    continue;
                                }
                            };

                            info!("Received data: {:?}", json_value);
                            *instance.good_packets.lock().await += 1;
                            instance.calculate_percentage().await;

                            if instance.tx.send(json_value).await.is_err() {
                                error!("Failed to send data to channel");
                            }

                            // Supprime le message traité du buffer
                            total_buffer = total_buffer[bytes_read + size as usize..].to_vec();
                        }
                    }
                    Ok(_) => continue,
                    Err(e) => {
                        error!("Error reading from UART: {}", e);
                    }
                }
            }
        })
    }

    pub async fn calculate_percentage(&self) -> f64 {
        let good_packets = *self.good_packets.lock().await;
        let bad_packets = *self.bad_packets.lock().await;
        let total_packets = good_packets + bad_packets;
        info!("Total packets: {}", total_packets);
        info!("Good packets: {}", good_packets);
        info!("Bad packets: {}", bad_packets);
        if total_packets == 0 {
            0.0
        } else {
            let percentage = (good_packets as f64 / total_packets as f64) * 100.0;
            *self.percentage.lock().await = percentage;
            info!("Good packet percentage: {:.2}%", percentage);

            percentage
        }
    }
    pub fn get_good_packet_percentage(&self) -> f64 {
        let percentage = self.percentage.blocking_lock();
        *percentage
    }

    pub fn get_bad_packet_count(&self) -> u64 {
        let bad_packets = self.bad_packets.blocking_lock();
        *bad_packets
    }

    pub fn get_good_packet_count(&self) -> u64 {
        let good_packets = self.good_packets.blocking_lock();
        *good_packets
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
