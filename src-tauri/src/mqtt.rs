use crate::constant::DATAS_NAMES;
use chrono::Local;
use log::{error, info};
use rumqttc::{Client, MqttOptions, QoS};
use serde_json::error;
use tokio::time::{sleep, Duration};
#[derive(Clone)]
pub struct MQTT {
    client: Client,

    topic: String,
}
impl MQTT {
    pub fn new() -> MQTT {
        let broker = "172.20.10.2";
        let port = 1883;
        let username = "nereides";
        let password = "raspberry";
        let topic = "testTopic".to_string();

        let mut mqttoptions = MqttOptions::new("rust_mqtt_client", broker, port);
        mqttoptions.set_credentials(username, password);
        let (client, mut eventloop) = Client::new(mqttoptions, 10);

        // Liste des topics
        let topics = DATAS_NAMES.clone();
        MQTT { client, topic }
    }

    pub fn send_event(&self, data_name: &str, data: f64) {
        let timestamp = Local::now().timestamp().to_string();
        let full_topic = format!("{}/{}/{}", self.topic, timestamp, data_name);
        let bytes: Vec<u8> = data.to_le_bytes().to_vec();

        if let Err(e) = self
            .client
            .publish(full_topic, QoS::AtLeastOnce, false, bytes)
        {
            error!("Erreur lors de l'envoi du message MQTT : {}", e);
        }
    }
}
