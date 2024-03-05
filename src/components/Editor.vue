<template>
  <div class="flex">
    <div class="mt-12 h-full w-full">
      <Codemirror
        :value="code"
        :options="cmOptions"
        ref="cmRef"
        @change="onChange"
        @input="onInput"
        @ready="onReady"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted, computed, type ComputedRef, watch } from "vue";
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
import { useStore } from "../stores/useStore";

const store = useStore();
const contents = ref(store.contents);
console.log(contents.value);
const message = ref("");
const cmRef = ref<CmComponentRef>();
const cmOptions: EditorConfiguration = {
  mode: "text/javascript",
  theme: "night",
  keyMap: 'sublime',
  extraKeys: {
      "Ctrl-Space": "autocomplete"
    },
};

const code = computed(() => store.contents);
console.log(code);
const onChange = (val: string, cm: Editor) => {
  console.log(val);
  console.log(cm.getValue());
  store.setContents(cm.getValue());
};

const onInput = (val: string) => {
  console.log(val);
  store.setContents(val);
};

const onReady = (cm: Editor) => {
  console.log(cm.focus());
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
