use indexmap::IndexMap;
use log::{error, info};
use paho_mqtt::{AsyncClient, ConnectOptionsBuilder, CreateOptionsBuilder, Message, QoS};
use std::{sync::Arc, time::Duration};
use tauri::async_runtime::spawn;

// Structure de l'acteur MQTT
pub struct MQTT {
    client: AsyncClient,
}

impl MQTT {
    pub fn new() -> Arc<MQTT> {
        let broker = "broker.hivemq.com";
        let port = 1883;

        let topic = "testTopic".to_string();
        let uri: String = format!("tcp://{}:{}", broker, port);
        let create_opts = CreateOptionsBuilder::new()
            .server_uri(uri)
            .client_id("rust_client2")
            .finalize();

        let client = AsyncClient::new(create_opts).unwrap();
        // Retourner l'acteur et le récepteur
        let instance = Arc::new(MQTT { client });
        instance.connect();
        instance
    }

    pub fn connect(self: &Arc<Self>) {
        let instance = self.clone();
        spawn(async move {
            loop {
                let conn_opts = ConnectOptionsBuilder::new()
                    .keep_alive_interval(Duration::from_secs(5))
                    .automatic_reconnect(Duration::from_secs(1), Duration::from_secs(60))
                    .clean_session(true)
                    //.password(password)
                    // .user_name(username)
                    .finalize();
                info!("Connecting to the MQTT broker");
                if let Err(e) = instance.client.connect(conn_opts).await {
                    error!("Error connecting to MQTT broker: {:?}", e);
                } else {
                    info!("Connected to the MQTT broker");
                    break;
                }
            }
        });
    }

    // Fonction pour envoyer un événement
    pub fn send_event(self: &Arc<Self>, data: IndexMap<&'static str, f64>) {
        if !self.client.is_connected() {
            info!("MQTT client is not connected, cannot send event.");
            return;
        }
        let instance = self.clone();
        spawn(async move {
            for (data_name, value) in data {
                let full_topic = format!("nereides/{}", data_name);
                let bytes: Vec<u8> = value.to_le_bytes().to_vec();
                let message = Message::new(full_topic.clone(), bytes.clone(), QoS::AtLeastOnce);
                if let Err(e) = instance.client.publish(message).await {
                    error!("Failed to publish message: {}", e);
                    return;
                }
                info!("Published message: {} to topic: {}", value, full_topic);
            }
        });
    }

    pub fn send(self: &Arc<Self>, data_name: String, value: f64) {
        let instance = self.clone();
        spawn(async move {
            if !instance.client.is_connected() {
                info!("MQTT client is not connected, cannot send event.");
                return;
            }
            let full_topic = format!("nereides/{}", data_name);
            let bytes: Vec<u8> = value.to_le_bytes().to_vec();
            let message = Message::new(full_topic.clone(), bytes.clone(), QoS::AtLeastOnce);
            if let Err(e) = instance.client.publish(message).await {
                error!("Failed to publish message: {}", e);
                return;
            }
            info!("Published message: {} to topic: {}", value, full_topic);
        });
    }
}
