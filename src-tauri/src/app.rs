use std::sync::Arc;
use std::time::Duration;

use log::info;

use serde::de;
use tauri::{async_runtime::spawn, AppHandle, Emitter};
use tokio::sync::Mutex;
use tokio::time::sleep;

use crate::csv_writer::Csv_writter;
use indexmap::IndexMap;

use crate::mqtt::MQTT;
use crate::uart_communication::UartCommunication;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use serde_json::Value;
use tokio::sync::mpsc::{channel, Receiver};
use crate::uart_communication::UartData;
#[derive(Clone)]
pub struct App {
    #[cfg(target_os = "linux")]
    uart_communication: Option<Arc<UartCommunication>>,

    #[cfg(not(target_os = "linux"))]
    uart_communication: Option<Arc<()>>, // Remplacer par un type générique ou un autre champ si nécessaire.
    app_handle: AppHandle,
    datas: Vec<&'static str>,
    mqtt: Arc<MQTT>,
    scv_writer: Csv_writter,
    data_api: IndexMap<&'static str, Option<f64>>,
    rx: Arc<Mutex<Receiver<UartData>>>,
}

impl App {
    pub fn new(app_handle: AppHandle) -> App {
        let datas = [
            "pac_temperature",
            "battery_voltage_v",
            "battery_current_a",
            "battery_soc",
            "battery_temp",
            "batterySE_temp",
            "motor_controller_temp",
            "mottor_current_a",
            "motor_voltage_v",
            "motor_rpm",
            "motor_throttle",
            "gps_long",
            "gps_lat",
            "motor_puissance_instantannée",
        ];

        // Initialisation d'un dictionnaire avec des noms de variables et des valeurs optionnelles
        let data_api_str = [
            "pac_temperature",
            "battery_voltage_v",
            "battery_current_a",
            "battery_soc",
            "battery_temp",
            "batterySE_temp",
            "motor_controller_temp",
            "mottor_current_a",
            "motor_voltage_v",
            "motor_rpm",
            "motor_throttle",
            "gps_long",
            "gps_lat",
            "motor_puissance_instantannée",
        ];
        let data_api: IndexMap<&'static str, Option<f64>> =
            data_api_str.iter().map(|&key| (key, None)).collect();
        let uart_communication = None;
        let mqtt = MQTT::new();
        let scv_writer = Csv_writter::new(data_api.clone());
        let (tx, rx) = channel::<UartData>(32);

        // S'assurer que le code avec socketcan est uniquement exécuté sur Linux
        #[cfg(target_os = "linux")]
        {
            let uart_communication = Some(UartCommunication::new("/dev/serial0", 115200, tx));
        }

        let instance = App {
            uart_communication,
            app_handle,
            datas: datas.to_vec(),
            mqtt,
            scv_writer,
            data_api,
            rx: Arc::new(Mutex::new(rx)),
        };
        instance.run();
        instance
    }

    pub fn treat_data(&mut self, data_name:&str , value: f64) {
        self.app_handle.emit(data_name, value).unwrap();
        self.update_mesures(data_name, value);
        if self.all_mesures_complete() {
            let data: IndexMap<&str, Option<f64>> = self.data_api.clone();
            let filtered_data: IndexMap<&str, f64> = data
                .iter()
                .filter_map(|(&key, &value)| value.map(|v| (key, v)))
                .collect();
            self.scv_writer.write_data(filtered_data.clone()).unwrap();
            self.mqtt.send_event(filtered_data);
            for value in self.data_api.values_mut() {
                *value = None;
            }
        }
    }

    pub fn treat_data_string(&mut self, data_name:&str , value: String) {
        self.app_handle.emit(data_name, value).unwrap();


    }
    pub fn update_mesures(&mut self, data_name: &str, value: f64) {
        if let Some(data) = self.data_api.get_mut(data_name) {
            *data = Some(value);
        }
    }

    pub fn all_mesures_complete(&self) -> bool {
        self.data_api.values().all(|val| val.is_some())
    }

    pub fn run(&self) {
        #[cfg(target_os = "linux")]
        {
            self.listen_uart_event();
        }
        #[cfg(target_os = "windows")]
        {
            self.start_sending_random_events();
        }
    }
    pub fn start_sending_random_events(&self) {
        // Cet fonction sert a envoyer des évenemnts pour chaque donnée avec une valeur aléatoire , pour l'instant nous ne gérons pas les évenement de type erre

        let mut rng = StdRng::from_entropy();
        let mut instance = self.clone();
        spawn(async move {
            info!("Démarrage de l'envoi des évenements aléatoires");

            loop {
                let datas = instance.datas.clone();
                for data_name in datas {
                    let value: f64 = rng.gen_range(0.0..100.0);
                    instance.treat_data(data_name, value);
                }
                sleep(Duration::from_millis(1000)).await;
            }
        });
    }
    #[cfg(target_os = "linux")]
    pub fn listen_uart_event(&self) {
        let rx = self.rx.clone();
        let mut instance = self.clone();
        spawn(async move {
            info!("Démarrage du traitement des JSON reçus");

            while let Some(json_value) = rx.lock().await.recv().await {
                match json_value{
                    UartData::Number(json_value) => {
                        instance.treat_data(&json_value.data_name, json_value.value);
                    
                }
                UartData::String(json_value) => {
                    instance.treat_data_string(&json_value.data_name, json_value.value);
                }
            }
            }
        });
        
    }

}
