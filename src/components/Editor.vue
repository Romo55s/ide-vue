<template>
  <div class="flex w-full h-full m-0 p-0 mx-0 my-0">
    <Codemirror
      :value="code"
      :options="cmOptions"
      ref="cmRef"
      height="100vh"
      width="100%"
      @change="onChange"
      @input="onInput"
      @ready="onReady"
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
const cmOptions: EditorConfiguration = {
  mode: "text/javascript",
  theme: "material-darker", // Use the "material-darker" theme
  keyMap: "sublime",
  extraKeys: {
    "Ctrl-Space": "autocomplete",
  },
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
