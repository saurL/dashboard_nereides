<script setup lang="ts">
import { Ref, ref } from "vue";
import verticalBar from "./component/verticalBar.vue";
import circularBar from "./component/circularBar.vue";
import { listen } from "@tauri-apps/api/event";

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
      dataRef.timestamp.value = event.payload.timestamp;
      dataRef.value.value = event.payload.value; // Traitement de l'événement
    }
  });
});
</script>

<template>
  <v-container class="pa-4" fluid>
    <v-col dense>
      <v-row>
        <!-- Batterie -->
        <v-col>
          <p>Batterie</p>
          <v-row>
            <verticalBar
              :name="'Tension'"
              :data_name="'battery_voltage_v'"
              :symbol="'V'"
            ></verticalBar>
            <verticalBar
              :name="'Courant '"
              :data_name="'battery_current_a'"
              :symbol="'A'"
            ></verticalBar>
          </v-row>
        </v-col>

        <!-- Moteur -->

        <v-row>
          <v-col>
            <p>Moteur</p>
            <v-row>
              <verticalBar
                :name="'Tension'"
                :data_name="'motor_voltage_v'"
                :symbol="'V'"
              ></verticalBar>
              <verticalBar
                :name="'Courant'"
                :data_name="'motor_current_a'"
                :symbol="'A'"
              ></verticalBar>
              <verticalBar
                :name="'Température'"
                :data_name="'motor_temp'"
                :symbol="'°C'"
              ></verticalBar>
            </v-row>
          </v-col>
        </v-row>

        <!-- PAC -->

        <v-row>
          <v-col>
            <p>PAC</p>

            <v-row>
              <verticalBar
                :name="'Tension'"
                :data_name="'pac_voltage_v'"
                :symbol="'V'"
              ></verticalBar>
              <verticalBar
                :name="'Courant'"
                :data_name="'pac_current_a'"
                :symbol="'V'"
              ></verticalBar>
              <verticalBar
                :name="'Température'"
                :data_name="'pac_temperature_c'"
                :symbol="'V'"
              ></verticalBar>
            </v-row>
          </v-col>
        </v-row>
      </v-row>

      <!-- GPS -->

      <v-row>
        <circularBar
          :name="'Vitesse'"
          :data_name="'gps_vitesse'"
          :symbol="'km/h'"
        ></circularBar>
        <verticalBar
          :name="'Throttle'"
          :data_name="'motor_throttle'"
        ></verticalBar>
        <circularBar
          :name="'RPM'"
          :data_name="'motor_rpm'"
          :symbol="'RPM'"
        ></circularBar>
      </v-row>
    </v-col>
  </v-container>
</template>

<style>
@import "vuetify/styles";

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
</style>
