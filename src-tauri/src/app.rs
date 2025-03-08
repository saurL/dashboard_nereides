use log::{error, info};

use tauri::{async_runtime::spawn, AppHandle, Emitter, Manager};

use crate::csv_writer::Csv_writter;
use crate::mqtt::MQTT;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
#[cfg(target_os = "linux")]
use socketcan::{CanSocket, EmbeddedFrame, Socket};
use tokio::time::{sleep, Duration};
pub struct App {
    #[cfg(target_os = "linux")]
    can_socket: Option<CanSocket>,

    #[cfg(not(target_os = "linux"))]
    can_socket: Option<()>, // Remplacer par un type générique ou un autre champ si nécessaire.
    app_handle: AppHandle,
    datas: Vec<&'static str>,
    mqtt: MQTT,
    scv_writer: Csv_writter,
}

impl App {
    pub fn new(app_handle: AppHandle) -> App {
        let datas = [
            "battery_voltage_v",
            "battery_current_a",
            "battery_soc",
            "battery_soh",
            "batterySE_temp",
            "motor_controller_temp",
            "motor_controller_status",
            "gps_millis",
            "gps_time",
            "gps_latitude",
            "gps_longitude",
            "gps_vitesse",
            "mottor_current_a",
            "motor_voltage_v",
            "motor_rpm",
            "motor_throttle",
            "motor_temp",
            "motor_error_code",
            "motor_switch_signals_status",
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
        let mut socket = None;
        let mqtt = MQTT::new();
        let scv_writer = Csv_writter::new();

        // S'assurer que le code avec socketcan est uniquement exécuté sur Linux
        #[cfg(target_os = "linux")]
        {
            if let Ok(can_socket) = CanSocket::open("can0") {
                socket = Some(can_socket);
            } else {
                error!("Impossible d'ouvrir le bus CAN");
            }
        }

        let instance = App {
            can_socket: socket,
            app_handle,
            datas: datas.to_vec(),
            mqtt,
            scv_writer,
        };
        instance.run();
        instance
    }

    pub fn run(&self) {
        #[cfg(target_os = "linux")]
        {
            self.read_can_data();
        }
        #[cfg(target_os = "windows")]
        {
            self.start_sending_random_events();
        }
    }
    pub fn start_sending_random_events(&self) {
        // Cet fonction sert a envoyer des évenemnts pour chaque donnée avec une valeur aléatoire , pour l'instant nous ne gérons pas les évenement de type erre
        let app_handle = self.app_handle.clone();
        let mut rng = StdRng::from_entropy();
        let datas = self.datas.clone();
        let mqtt = self.mqtt.clone();
        let mut scv_writer = self.scv_writer.clone();
        spawn(async move {
            info!("Démarrage de l'envoi des évenements aléatoires");

            loop {
                for data in &datas {
                    let value: f64 = rng.gen_range(0.0..100.0);
                    app_handle.emit(data, value).unwrap();
                    mqtt.send_event(data, value);
                    scv_writer.write_data(data, value).unwrap();
                }
                sleep(Duration::from_secs(1)).await;
            }
        });
    }
    #[cfg(target_os = "linux")]
    pub fn read_can_data(&self) {
        let can_socket = self.socket.clone();

        if let Some(socket) = can_socket {
            let app_handle = self.app_handle.clone();

            spawn(async move {
                info!("Démarrage de la lecture des données CAN");

                loop {
                    match can_socket.read_frame() {
                        Ok(frame) => {
                            let id = frame.id();
                            let data = frame.data();
                            info!("Donnée reçue {:?} {:?}", id, data);
                            // Traitez les données du frame ici et émettez des événements en conséquence
                            // Par exemple, vous pouvez convertir les données en une valeur et émettre un événement
                            let value = data[0] as f64; // Conversion simplifiée pour l'exemple
                            app_handle.emit("donnée", value).unwrap();
                        }
                        Err(e) => {
                            error!("Erreur lors de la lecture du frame CAN: {:?}", e);
                        }
                    }
                    sleep(Duration::from_millis(100)).await; // Ajustez la fréquence de lecture si nécessaire
                }
            });
        }
    }
}
