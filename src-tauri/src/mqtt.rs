use log::{error, info};
use rumqttc::{Client, MqttOptions, QoS};
use serde_json::error;
use std::time::{SystemTime, UNIX_EPOCH};
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
        let topics = vec![
            "gps_millis",
            "gps_time",
            "gps_latitude",
            "gps_longitude",
            "gps_vitesse",
            "motor_current_a",
            "motor_voltage_v",
            "motor_rpm",
            "motor_throttle",
            "motor_temp",
            "motor_controller_temp",
            "motor_error_code",
            "motor_controller_status",
            "motor_switch_signals_status",
            "battery_voltage_v",
            "battery_current_a",
            "battery_soc",
            "battery_soh",
            "batterySE_temp",
            "pac_emergency_stop",
            "pac_start",
            "pac_stop",
            "pac_current_a",
            "pac_voltage_v",
            "pac_system_state",
            "pac_error_flag",
            "pac_hydrogen_consumption_mgs",
            "pac_temperature_c",
            "pac_system_errors",
            "pac_fan_error",
            "pac_operation_time",
            "pac_produced_energy",
            "pac_total_operation_time",
            "pac_total_produced_energy",
        ];

        MQTT { client, topic }
    }

    pub fn send_event(&self, data_name: &str, data: f64) {
        let timestamp = get_time();
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

// Fonction pour obtenir le timestamp actuel
fn get_time() -> String {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    format!("D{}", since_the_epoch.as_secs())
}
