<template>
    <div ref="terminal" class="terminal fixed bg-black"></div>
</template>

<script setup lang="ts">
import { Terminal } from "xterm";
import "xterm/css/xterm.css";
import { ref, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

const terminalRef = ref(null);
var terminalInstance = new Terminal();
terminalInstance.write("Hello world!");

onMounted(() => {
  if (terminalRef.value) {
    terminalInstance.open(terminalRef.value);
  }

  // Establecer la comunicación con el backend de Tauri
  terminalInstance.onData((data) => {
    // Enviar comandos al backend de Tauri
    invoke("run_command", { command: data })
      .then((response) => {
        // Mostrar los resultados en la terminal
        terminalInstance.write(response + "\n");
      })
      .catch((error) => {
        terminalInstance.write("Error: " + error + "\n");
      });
  });
});
</script>

<style scoped>
.terminal-container {
  width: 100%;
  height: 100%;
}

.terminal {
  width: 50%;
  height: 50%;
}
</style>
