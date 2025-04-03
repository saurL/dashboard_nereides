use crate::constant::DATAS_NAMES;
use chrono::Local;
use indexmap::IndexMap;
use log::{error, info};
use mpsc::{channel, Receiver, Sender};
use rumqttc::{AsyncClient, EventLoop, MqttOptions, QoS};
use std::sync::mpsc;
use std::thread::spawn;
use std::{thread, time::Duration};
use tauri::Event;

// Message types pour l'actor
pub enum MqttMessage {
    SendEvent(IndexMap<&'static str, f64>),
    Terminate,
}

// Structure de l'acteur MQTT
pub struct MQTTActor {
    eventloop: EventLoop,
    client: AsyncClient,
    topic: String,
    receiver: Receiver<MqttMessage>,
}

impl MQTTActor {
    pub fn new(receiver: Receiver<MqttMessage>) -> (MQTTActor) {
        let broker = "broker.hivemq.com";
        let port = 1883;
        let username = "nereides";
        let password = "raspberry";
        let topic = "testTopic".to_string();

        let mut mqttoptions = MqttOptions::new("rust_mqtt_client", broker, port);
        mqttoptions.set_keep_alive(Duration::from_secs(3));
        let (client, mut eventloop) = AsyncClient::new(mqttoptions, 10);

        // Retourner l'acteur et le récepteur
        (MQTTActor {
            client,
            eventloop,
            topic,
            receiver,
        })
    }

    // Fonction pour envoyer un événement
    fn send_event(&mut self, data: IndexMap<&'static str, f64>) {
        info!("send event called in actor");

        let timestamp = Local::now().timestamp().to_string();
        for (data_name, value) in data {
            let full_topic = format!("{}/{}", self.topic, "nereides");
            let bytes: Vec<u8> = value.to_le_bytes().to_vec();
            info!("envoit des données : {} : {}", data_name, value);

            if let Err(e) = self
                .client
                .publish(full_topic.clone(), QoS::AtLeastOnce, false, bytes)
            {
                error!("Erreur lors de l'envoi du message MQTT : {}", e);
            } else {
                info!("Message envoyé à {} : {}", full_topic, value);
            }
            info!("après le message envoyé");
            for (i, notification) in self.connection.iter().enumerate() {
                match notification {
                    Ok(notification) => {
                        println!("Notification = {:?}", notification);
                    }
                    Err(e) => {
                        error!("Erreur lors de la réception de la notification : {}", e);
                    }
                }
            }
        }
        info!("fin de la boucle");
    }

    // Boucle de traitement des messages
    fn handle_messages(&mut self) {
        loop {
            match self.receiver.recv() {
                Ok(MqttMessage::SendEvent(data)) => {
                    info!("Envoi d'un événement");
                    self.send_event(data);
                    info!("Evénement envoyé");
                }
                Ok(MqttMessage::Terminate) => {
                    info!("Arrêt de l'acteur MQTT.");
                    break;
                }
                Err(e) => {
                    error!("Erreur dans la réception du message : {}", e);
                    break;
                }
            }
        }
        info!("FIn de la boucle event loop");
    }

    // Fonction pour démarrer le thread de l'acteur
    pub fn start_actor(mut self) -> thread::JoinHandle<()> {
        thread::spawn(move || {
            self.handle_messages();
        })
    }
}

#[derive(Clone)]
pub struct MQTThandle {
    sender: Sender<MqttMessage>,
}

impl MQTThandle {
    pub fn new() -> Self {
        let (sender, receiver) = channel();

        let actor = MQTTActor::new(receiver);
        actor.start_actor();

        Self { sender }
    }

    pub fn send_event(&self, datas: IndexMap<&'static str, f64>) {
        info!("send event called");
        self.sender.send(MqttMessage::SendEvent(datas)).unwrap();
    }
}
