<template>
  <div class="column data_block">
    <p v-if="label">{{ label }}</p>
    <p class="coloredData" :style="{ color: computedColor }">
      {{ roundToTwo(displayed_data) }} {{ symbol }}
    </p>
  </div>
</template>

<script setup lang="ts">
import { computed, ref } from "vue";
import { listen } from "@tauri-apps/api/event";
import { info } from "@tauri-apps/plugin-log";
const props = defineProps({
  data_name: {
    type: String,
    required: true,
  },
  symbol: {
    type: String,
    default: undefined,
  },
  label: {
    type: String,
    default: undefined,
  },
  maxValue: {
    type: Number,
    default: 100,
  },
  dangerStartValue: {
    type: Number,
    default: undefined,
  },
  dangerMaxValue: {
    type: Number,
    default: undefined,
  },
  minValue: {
    type: Number,
    default: 0,
  },
});
const computedDangerMaxValue = computed(() => {
  return props.dangerMaxValue !== undefined
    ? props.dangerMaxValue
    : props.maxValue;
});
const range = computed(() => computedDangerMaxValue.value - props.minValue);

const displayed_data = ref(0);
listen(props.data_name, (event) => {
  info(`Received event: ${props.data_name}`);
  console.log(event);
  displayed_data.value = event.payload as number; // Traitement de l'événement
});
function roundToTwo(num: number) {
  return Math.round(num * 100) / 100;
}

const computedColor = computed(() => {
  const value = displayed_data.value;

  const green = Math.floor((1 - (value - props.minValue) / range.value) * 255);
  const red = Math.floor(((value - props.minValue) / range.value) * 255);

  return `rgb(${red}, ${green}, 0)`;
});
</script>
<style scoped>
p {
  margin: 0;
  padding: 0;
  text-align: center;
}
.coloredData {
  text-align: right;
  min-width: 68px;
}
.row {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 5px;
}

.column {
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
  margin-bottom: 5px;
}

.data_block {
  padding-left: 2px;
  padding-right: 2px;
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
}
</style>
