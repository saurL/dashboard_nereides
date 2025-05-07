<script setup lang="ts">
import { Ref, ref, onMounted } from "vue";
import coloredData from "./component/coloredData.vue";
import { listen } from "@tauri-apps/api/event";
import ColoredData from "./component/coloredData.vue";

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
const dataRefs = new Map<
  string,
  { timestamp: Ref<string, string>; value: Ref<number, number> }
>();

datas.forEach((data) => {
  const timestamp = ref("");
  const value = ref(0);
  dataRefs.set(data, { timestamp: timestamp, value: value });
  listen(data, (event) => {
    const dataRef = dataRefs.get(data);
    if (dataRef) {
      const payload = event.payload as { timestamp: string; value: number };
      dataRef.timestamp.value = payload.timestamp;
      dataRef.value.value = payload.value; // Traitement de l'événement
    }
  });
});
onMounted(async () => {});
</script>

<template>
  <div class="box">
    <div class="top-container">
      <!-- Batterie -->
      <div class="row">
        <div class="data">
          <p>BATTERIE</p>
          <div class="row">
            <p>Energie</p>
            <coloredData
              :label="'Charge'"
              :data_name="'battery_soc'"
              :symbol="'%'"
            ></coloredData>
            <coloredData
              :label="'Température'"
              :data_name="'battery_temp'"
              :symbol="'°C'"
            ></coloredData>
            <coloredData
              :label="'Intensité'"
              :data_name="'battery_current_a'"
              :symbol="'A'"
            ></coloredData>
          </div>
          <div class="row">
            <p>SE</p>
            <coloredData
              :data_name="'battery_soc_SE'"
              :symbol="'%'"
            ></coloredData>
            <coloredData
              :data_name="'battery_temp_SE'"
              :symbol="'°C'"
            ></coloredData>
            <coloredData
              :label="'Intensité'"
              :data_name="'batterySE_current_a'"
              :symbol="'A'"
            ></coloredData>
          </div>
        </div>
        <!-- Moteur -->

        <div class="data">
          <p>MOTEUR</p>
          <div class="row">
            <coloredData
              :label="'Puissance'"
              :data_name="'motor_pow'"
              :symbol="'W'"
            ></coloredData>

            <coloredData
              :label="'Ampérage'"
              :data_name="'mottor_current_a'"
              :symbol="'A'"
            ></coloredData>
          </div>
          <div class="row">
            <coloredData
              :label="'Throttle'"
              :data_name="'motor_throttle'"
              :symbol="'%'"
            ></coloredData>
            <coloredData
              :label="'Température'"
              :data_name="'motor_temp'"
              :symbol="'°C'"
            ></coloredData>
          </div>
        </div>
      </div>

      <!-- PAC -->
      <div class="row">
        <div class="data">
          <p>PAC</p>
          <div class="row">
            <coloredData
              :label="'H2'"
              :data_name="'qtt_H2rest'"
              :symbol="'L'"
            ></coloredData>
          </div>
          <div class="row">
            <coloredData
              :label="'Température'"
              :data_name="'pac_temperature'"
              :symbol="'°C'"
            ></coloredData>
          </div>
        </div>
        <!-- GPS -->
        <div class="data">
          <div class="row">
            <coloredData
              :label="'RPM'"
              :data_name="'motor_rpm'"
              :symbol="'RPM'"
            ></coloredData>
          </div>
          <div class="row">
            <coloredData
              :label="'Vitesse'"
              :data_name="'gps_vitesse'"
              :symbol="'km/h - noeuds '"
            ></coloredData>

            <coloredData
              :label="'Heure'"
              :data_name="'Heure'"
              :symbol="'HEURE'"
            ></coloredData>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<style>
.box {
}

.data {
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
  margin: 10px;
  padding: 10px;
  border-radius: 10px;
  background-color: white;
  box-shadow: 0px 4px 8px rgba(0, 0, 0, 0.1);
}

.top-container {
  /*
  display: flex;
  flex: wrap; */
  align-content: center;
  border: 2px solid white;
}

:root {
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  font-size: 16px;
  line-height: 24px;
  font-weight: 400;

  color: #0f0f0f;
  background-color: #f6f6f6;

  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}
html,
body {
  cursor: none;
  margin: 0;
  padding: 0;
  overflow: hidden;
  width: 100%;
  height: 100%;
  -ms-overflow-style: none; /* IE and Edge */
  scrollbar-width: none; /* Firefox */
  background-color: black;
  color: white;
}

/* Hide scrollbar for Chrome, Safari and Opera */
html::-webkit-scrollbar {
  display: none;
}
.v-card {
  color: black;
}
.icon {
  width: 20px;
  height: 20px;
}

.row {
  display: flex;
  flex-direction: row;
  justify-content: space-between;
  align-items: flex-end;
  margin-bottom: 10px;
}
p {
  margin: 0;
  padding: 0;
  font-size: 14px;
  color: black;
}
</style>
