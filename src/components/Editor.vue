<template>
  <div class="flex">
    <div style="margin-top: 3%">
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
import Codemirror from "codemirror-editor-vue3";
import type { CmComponentRef } from "codemirror-editor-vue3";
import type { Editor, EditorConfiguration } from "codemirror";
import "codemirror/mode/javascript/javascript.js";

import { useStore } from "../stores/useStore";

const store = useStore();
const contents = ref(store.contents);
console.log(contents.value);
//let code = computed(() => contents.value.split(/\r\n/));
/*const code = ref(
  `var i = 0;
      for (; i < 9; i++) {
          console.log(i);
          // more statements
      }
      `
);*/

const cmRef = ref<CmComponentRef>();
const cmOptions: EditorConfiguration = {
  mode: "text/javascript",
  theme: "default",
};

let code = contents.value;

console.log(code);
const onChange = (val: string, cm: Editor) => {
  console.log(val);
  console.log(cm.getValue());
};

const onInput = (val: string) => {
  console.log(val);
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
