use crate::constant::DATAS_NAMES;
use chrono::Local;
use indexmap::IndexMap;
use log::error;
use rumqttc::{Client, MqttOptions, QoS};
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

        MQTT { client, topic }
    }

    pub fn send_event(&self, data: IndexMap<&'static str, f64>) {
        let timestamp = Local::now().timestamp().to_string();
        for (data_name, value) in data {
            let full_topic = format!("{}/{}/{}", self.topic, timestamp, data_name);
            let bytes: Vec<u8> = value.to_le_bytes().to_vec();

            if let Err(e) = self
                .client
                .publish(full_topic, QoS::AtLeastOnce, false, bytes)
            {
                error!("Erreur lors de l'envoi du message MQTT : {}", e);
            }
        }
    }
}
