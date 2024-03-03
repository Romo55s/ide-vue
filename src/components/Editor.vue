<template>
  <div>
    <NavBar v-if="navBarExists" />
    <div class="flex">
      <SideBar v-if="sideBarExists" />
      <div style="margin-top: 3%">
        <Codemirror
          v-model:value="code"
          :options="cmOptions"
          border
          ref="cmRef"
          height="1280px"
          width="100vw"
          @change="onChange"
          @input="onInput"
          @ready="onReady"
        />
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted } from "vue";
import "codemirror/mode/javascript/javascript.js";
import Codemirror from "codemirror-editor-vue3";
import type { CmComponentRef } from "codemirror-editor-vue3";
import type { Editor, EditorConfiguration } from "codemirror";
import "codemirror/mode/javascript/javascript.js";

const code = ref(
  `var i = 0;
      for (; i < 9; i++) {
          console.log(i);
          // more statements
      }
      `
);
const cmRef = ref<CmComponentRef>();
const cmOptions: EditorConfiguration = {
  mode: "text/javascript", theme: 'default',
};

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

// Verificar si el NavBar y el SideBar existen
const navBarExists = !!document.querySelector("header");
const sideBarExists = !!document.querySelector(".bg-gray-800");
</script>
