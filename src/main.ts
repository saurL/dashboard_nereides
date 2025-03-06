import { createApp } from "vue";
import App from "./App.vue";
import { attachConsole } from '@tauri-apps/plugin-log';
attachConsole()


const app = createApp(App);

app.mount("#app");
