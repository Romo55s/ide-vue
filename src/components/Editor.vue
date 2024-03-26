<template>
  <div class="flex flex-col w-full h-full z-10 flex-1 min-w-full overflow-hidden flex-grow-0">
    <div class="flex w-full h-full min-w-full overflow-hidden flex-grow-0">
      <Codemirror
        :value="code"
        :options="cmOptions"
        ref="cmRef"
        height=""
        @change="onChange"
        @input="onInput"
        @ready="onReady"
        @cursorActivity="onCursorActivity"
      />
      <div class="">
        <nav class="">
          <!-- Ajuste de margen superior y lateral -->
          <ul class="flex bg-black min-h-full text-green-400 flex-20 overflow-y-auto">
            <li class="border border-white p-2">
              <router-link to="/analizer/lexic" class="hover:text-gray-300"
                >Lexic</router-link
              >
            </li>
            <li class="border border-white p-2">
              <router-link to="/analizer/errors" class="hover:text-gray-300"
                >Errors</router-link
              >
            </li>
            <li class="border border-white p-2">
              <router-link to="/analizer/semantic" class="hover:text-gray-300"
                >Semantic</router-link
              >
            </li>
            <li class="border border-white p-2">
              <router-link to="/analizer/syntax" class="hover:text-gray-300"
                >Syntax</router-link
              >
            </li>
          </ul>
        </nav>
        <div class="w-full h-full overflow-y-auto">
          <Analizers />
        </div>
      </div>
    </div>
    <Terminal class="w-full" />
  </div>
</template>

<script setup lang="ts">
import { Lexic, Errors, Semantic, Syntax } from "../views";
import Terminal from "./Terminal.vue";
import {
  ref,
  onMounted,
  onUnmounted,
  computed,
  type ComputedRef,
  watch,
} from "vue";
import "codemirror/mode/javascript/javascript.js";
import "codemirror/keymap/sublime.js";
import Codemirror from "codemirror-editor-vue3";
import type { CmComponentRef } from "codemirror-editor-vue3";
import type { Editor, EditorConfiguration } from "codemirror";
import "codemirror/mode/javascript/javascript.js";
import "codemirror/addon/hint/show-hint.js"; // Importa el addon para el autocompletado
import "codemirror/addon/hint/javascript-hint.js"; // Importa el addon para el autocompletado de JavaScript
import "codemirror/lib/codemirror.css";
import "codemirror/theme/night.css";
import "codemirror/theme/material-darker.css"; // Import the dark theme
import { useStore } from "../stores/useStore";
import Analizers from "../views/Analizers.vue";

const store = useStore();
const contents = ref(store.contents);
console.log(contents.value);
const message = ref("");
const cmRef = ref<CmComponentRef>();
store.setFlagEditor(true);
const cmOptions: EditorConfiguration = {
  mode: "text/javascript",
  theme: "material-darker", // Use the "material-darker" theme
  keyMap: "sublime",
  extraKeys: {
    "Ctrl-Space": "autocomplete",
  },
};

const sidebarWidth = computed(() => store.sidebarWidth);
const isCollapsed = computed(() => store.collapsed);

const onCursorActivity = (cm: Editor) => {
  const cursor = cm.getCursor(); // Get the current cursor position
  store.setColumn(cursor.ch); // Update the store with the current column
  store.setRow(cursor.line); // Update the store with the current row (line)
};

const code = computed(() => store.contents);

const onChange = (val: string, cm: Editor) => {
  store.setContents(cm.getValue());
};

const onInput = (val: string) => {
  store.setContents(val);
};

const onReady = (cm: Editor) => {};

onMounted(() => {
  setTimeout(() => {
    cmRef.value?.refresh();
  }, 1000);

  setTimeout(() => {
    cmRef.value?.cminstance.isClean();
  }, 3000);
});

onUnmounted(() => {
  cmRef.value?.destroy();
});
</script>
