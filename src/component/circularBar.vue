<template>
  <div class="progress-container">
    <p>{{ name }}</p>
    <div class="circular-progress">
      <v-progress-circular
        :model-value="displayed_data"
        :color="computedColor"
        :size="200"
        :width="12"
        :rotate="180"
        height="20"
      >
        <template v-slot:default
          >{{ roundToTwo(displayed_data) }} {{ symbol }}
        </template>
      </v-progress-circular>
    </div>
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
.circular-progress {
  position: relative;
  width: 100%;
  height: 300px;
  display: flex;

  align-items: center;
  justify-content: center;
}
.progress-container {
  height: min-content;
  width: 200px;
  flex-direction: column;
  display: flex;
  align-items: center;
  justify-content: center;
}
.v-progress-linear__determinate {
  animation-duration: 3s;
}

.indictor {
  position: absolute;
}
</style>
