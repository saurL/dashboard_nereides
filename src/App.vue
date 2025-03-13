<script setup lang="ts">
import { Ref, ref,onMounted } from "vue";
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
onMounted(async () => {
})

</script>

<template>
  <div class="box">
    <div class="top-container">
      <!-- Batterie -->

      <div class="data">
        <img
          class="icon"
          src="./assets/icons8-nombreuses-batteries-90.png"
          alt=""
        />

        <coloredData
          :data_name="'battery_voltage_v'"
          :symbol="'V'"
        ></coloredData>
        <coloredData
          :data_name="'battery_current_a'"
          :symbol="'A'"
        ></coloredData>
      </div>

      <!-- Moteur -->

      <div class="data">
        <img class="icon" src="./assets/icons8-moteur-90.png" alt="" />

        <coloredData :data_name="'motor_voltage_v'" :symbol="'V'"></coloredData>
        <coloredData :data_name="'motor_current_a'" :symbol="'A'"></coloredData>
        <coloredData :data_name="'motor_temp'" :symbol="'°C'"></coloredData>
      </div>

      <!-- PAC -->

      <div class="data">
        <img class="icon" src="./assets/icons8-hydrogène-100.png" alt="" />
        <coloredData :data_name="'pac_voltage_v'" :symbol="'V'"></coloredData>
        <coloredData :data_name="'pac_current_a'" :symbol="'V'"></coloredData>
        <coloredData
          :data_name="'pac_temperature_c'"
          :symbol="'°C'"
        ></coloredData>
      </div>
    </div>

    <!-- GPS -->
  </div>
</template>

<style>


.box {
  
}

.data {
}

.top-container {/*
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
</style>
