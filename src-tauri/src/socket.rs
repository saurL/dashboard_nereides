use serde_json::Value;
use std::sync::Arc;
use tauri::async_runtime::spawn;
use tauri::{AppHandle, Emitter};
use tokio::io::AsyncReadExt;
use tokio::net::TcpListener;
use tokio::sync::Mutex;
#[derive(Debug)]
pub struct SocketServer {
    listener: TcpListener,
    app_handle: Arc<Mutex<AppHandle>>,
    buffer: Mutex<Vec<u8>>,
}

impl SocketServer {
    pub async fn new(
        port: u16,
        app_handle: Arc<Mutex<AppHandle>>,
    ) -> Result<SocketServer, Box<dyn std::error::Error>> {
        let address = format!("127.0.0.1:{}", port);
        log::info!("Starting server at {}", address);
        let listener = TcpListener::bind(address.clone()).await?;
        let instance = Self {
            listener,
            app_handle,
            buffer: Mutex::new(Vec::new()),
        };
        Ok(instance)
    }

    pub fn run(mut self: Arc<Self>) {
        spawn(async move {
            log::info!("Server started");

            loop {
                log::info!("Waiting for connection...");

                let (mut socket, _) = self.listener.accept().await.unwrap();
                log::info!("Accepted connection from {:?}", socket.peer_addr().unwrap());

                let instance = self.clone();
                spawn(async move {
                    loop {
                        let mut buffer = [0; 1024];
                        match socket.read(&mut buffer).await {
                            Ok(0) => return, // Connection closed
                            Ok(n) => {
                                instance.buffer.lock().await.extend_from_slice(&buffer[..n]);
                                while let Some(packet) = instance.read_packet().await {
                                    if !packet.is_empty() {
                                        let packet_str = String::from_utf8_lossy(&packet);
                                        let json_value: serde_json::Value =
                                            match serde_json::from_str(&packet_str) {
                                                Ok(value) => value,
                                                Err(e) => {
                                                    log::error!(
                                                        "Failed to parse packet as JSON: {:?}",
                                                        e
                                                    );
                                                    continue;
                                                }
                                            };
                                        instance.send_event(json_value).await;
                                    } else {
                                        break;
                                    }
                                }
                            }

                            Err(e) => {
                                eprintln!("Failed to read from socket; err = {:?}", e);
                                return;
                            }
                        }
                    }
                });
            }
        });
    }
    async fn send_event(&self, json_value: Value) {
        if let (Some(event_name), Some(value)) = (
            json_value.get("data").and_then(|v| v.as_str()),
            json_value.get("value").and_then(|v| v.as_f64()),
        ) {
            let app_handle = self.app_handle.lock().await;
            log::info!("Sending event: {} with data: {:?}", event_name, value);
            app_handle.emit(&event_name, value).unwrap();
        } else {
            log::error!("Invalid packet: {:?}", json_value);
        }
    }

    pub async fn decode_varint(&self, buff: Vec<u8>) -> Result<(u64, usize), String> {
        let mut value: u64 = 0;
        let mut shift: u32 = 0;
        let mut bytes_read: usize = 0;
        for &byte in buff.iter() {
            bytes_read += 1;

            value |= ((byte & 0x7f) as u64) << shift;
            if (byte & 0x80) == 0 {
                return Ok((value, bytes_read));
            }

            shift += 7;
            if shift >= 64 {
                return Err("Varint is too long".to_string());
            }
        }

        Err("Buffer ended prematurely".to_string())
    }

    pub async fn read_packet(&self) -> Option<Vec<u8>> {
        let mut buff = self.buffer.lock().await;
        if buff.is_empty() {
            return None;
        }

        // Décoder la taille du paquet
        let (size, size_length) = match self.decode_varint(buff.to_vec()).await {
            Ok(result) => result,
            Err(_) => return None,
        };

        // Vérifier si le message est complet
        if size_length == 0 || size as usize > buff.len() - size_length {
            return None;
        }

        // Extraire le paquet complet du buffer
        let packet = buff[size_length..size_length + size as usize].to_vec();

        // Mettre à jour le buffer en supprimant le message traité
        buff.drain(..size_length + size as usize);

        // Émettre un événement 'packet' si nécessaire

        Some(packet)
    }
}
