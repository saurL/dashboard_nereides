use std::sync::LazyLock;
#[cfg(target_os = "windows")]
pub static CSV_DIR_PATH: &str = "C:\\nereides_data\\";

#[cfg(target_os = "linux")]
pub static CSV_DIR_PATH: &str = "/home/nereides/.local/share/mon_app/";

pub static SCV_FILE_NAME: &str = "nereides_data";

pub static DATAS_NAMES: LazyLock<Vec<&str>> = LazyLock::new(|| {
    vec![
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
        "pac_total_produced_energy",
    ]
});
