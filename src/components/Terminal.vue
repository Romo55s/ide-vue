<template>
  <div
    ref="terminal"
    class="h-41 w-full bg-neutral-900 text-white p-4 overflow-y-auto border-t-8 border-neutral-950 font-consolas"
  >
    <div class="mb-2">
      <span class="text-green-400">usuario@host:</span>
      <span class="text-blue-400">{{ ruta }}</span>
      <span class="text-white"> $</span>
      <input
        ref="input"
        type="text"
        v-model="userInput"
        class="bg-transparent border-none focus:outline-none text-white"
        @keydown.enter="processInput"
      />
    </div>
    <div v-for="(line, index) in terminalLines" :key="index">
      <span v-if="index === currentLineIndex" class="text-yellow-400">{{
        line
      }}</span>
      <span v-else>{{ line }}</span>
      <br />
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from "vue";

const ruta = ref("");
const terminalLines = ref<string[]>([]);
const currentLineIndex = ref(0);
const userInput = ref("");

const randomizeRoute = () => {
  const directorios = [
    "~/documentos",
    "~/descargas",
    "~/proyectos",
    "~/fotos",
    "~/videos",
  ];
  ruta.value = directorios[Math.floor(Math.random() * directorios.length)];
};

const processInput = () => {
  terminalLines.value.push(userInput.value);
  userInput.value = "";
};

const simulateTextStutter = () => {
  setInterval(() => {
    currentLineIndex.value = Math.floor(
      Math.random() * terminalLines.value.length
    );
  }, 2000);
};

randomizeRoute();
simulateTextStutter();
</script>
