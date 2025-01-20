<template>
  <div class="progress-container">
    <p>{{ name }}</p>
    <div class="vertical-progress-wrapper">
      <div class="vertical-progress">
        <v-progress-linear
          :model-value="displayed_data"
          :color="computedColor"
          height="20"
        >
        </v-progress-linear>
      </div>
    </div>

    <p :style="{ color: computedColor }">
      {{ roundToTwo(displayed_data) }} {{ symbol }}
    </p>
  </div>
</template>

<script setup lang="ts">
import { computed, ref } from "vue";
import { listen } from "@tauri-apps/api/event";
const props = defineProps({
  name: {
    type: String,
    required: true,
  },
  data_name: {
    type: String,
    required: true,
  },
  symbol: {
    type: String,
    default: undefined,
  },
  maxValue: {
    type: Number,
    default: 100,
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
  console.log(event);
  displayed_data.value = event.payload.value; // Traitement de l'événement
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
.vertical-progress-wrapper {
  width: 200px;
  height: 200px;
  display: flex;
  align-items: center;
  justify-content: center;
}
.vertical-progress {
  width: 400px;
  height: 100px;
  display: flex;

  align-items: center;
  justify-content: center;
  transform: rotate(-90deg);
}
.progress-container {
  height: min-content;
  width: 100px;
  flex-direction: column;
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 10px;
}
.v-progress-linear__determinate {
  animation-duration: 3s;
}
</style>
