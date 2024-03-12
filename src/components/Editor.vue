<template>
  <div class="flex overflow-hidden" >
    <Codemirror
      :value="code"
      :options="cmOptions"
      ref="cmRef"
      height="100vh"
      width="100%"
      @change="onChange"
      @input="onInput"
      @ready="onReady"
      @cursorActivity="onCursorActivity" 
    />

  </div>
</template>

<script setup lang="ts">
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

const onReady = (cm: Editor) => {

};



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
