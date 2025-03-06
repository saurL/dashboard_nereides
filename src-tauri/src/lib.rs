mod csv;
mod app;
use std::sync::Arc;
use log::info;
use tokio::sync::Mutex;
use app::App;
use tauri::{AppHandle, Manager};
use tauri::{async_runtime::spawn, Emitter};
use tokio::time::{sleep, Duration};
// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn async_main(app_handle: Arc<Mutex<AppHandle>>) -> Result<(), Box<dyn std::error::Error>> {
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
    for data in datas {
        let app_handle = app_handle.clone();
        spawn(async move {
            loop {
                if let Err(e) = check_data(data, app_handle.clone()).await {
                    log::error!(
                        "{}",
                        format!("Error in check_data for data {}: {}. Retrying...", data, e)
                    );
                } else {
                    break;
                }
            }
        });
    }

    Ok(())
}

async fn check_data(
    data_name: &str,
    app_handle: Arc<Mutex<AppHandle>>,
) -> Result<(), Box<dyn std::error::Error + Send>> {
    let dir_path = "C:\\Programmation\\Projets\\Nereide\\dashboard\\csv";
    let file_path = format!("{}\\{}.csv", dir_path, data_name);

    let mut old_data: Vec<csv::structs::Line> = csv::reader::read_csv(&file_path)?;

    loop {
        let data: Vec<csv::structs::Line> = csv::reader::read_csv(&file_path)?;
        let last_data = data.last();
        if last_data != old_data.last() {
            let sent_data = last_data.unwrap();
            send_event(app_handle.clone(), data_name, &sent_data).await;
        }

        old_data = data;
        sleep(Duration::from_millis(500)).await;
    }
}
async fn send_event(
    app_handle: Arc<Mutex<AppHandle>>,
    event: &str,
    data: &(impl serde::Serialize + std::fmt::Debug),
) {
    let app_handle = app_handle.lock().await;
    app_handle.emit(event, data).unwrap();
    let string = format!("Event sent: {} , with data : {:?}", event, data);
    log::info!("{}", string);
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {

    tauri::Builder::default()
    .setup(|app| {

        app.manage(App::new(app.handle().clone()));

        Ok(())
    })
        .plugin(tauri_plugin_shell::init())
        .plugin(
            tauri_plugin_log::Builder::new()
                .target(tauri_plugin_log::Target::new(
                    tauri_plugin_log::TargetKind::LogDir {
                        file_name: Some("logs".to_string()),
                    },
                )).target(tauri_plugin_log::Target::new(
                    tauri_plugin_log::TargetKind::Webview,
                  ))
                .build(),
        )
        .invoke_handler(tauri::generate_handler![greet])
       
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
