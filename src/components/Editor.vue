<template>
  <div
    class="flex flex-col w-full h-full z-10 flex-1 min-w-full overflow-hidden flex-grow-0"
  >
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
          <ul
            class="flex bg-neutral-950 min-h-full text-green-400 w-full overflow-y-auto"
          >
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
import { defineMode } from "codemirror";

const store = useStore();
const contents = ref(store.contents);
console.log(contents.value);
const message = ref("");
const cmRef = ref<CmComponentRef>();
store.setFlagEditor(true);

defineMode("customMode", () => {
  return {
    token(stream, state) {
      const keywords = [
        "if",
        "else",
        "fn",
        "do",
        "while",
        "switch",
        "case",
        "integer",
        "int",
        "string",
        "float",
        "fl",
        "db",
        "double",
        "end",
        "main",
        "function",
        "and",
        "or",
      ];
      const operators = [
        "+",
        "-",
        "*",
        "/",
        "%",
        "^",
        "<",
        "<=",
        ">",
        ">=",
        "!=",
        "==",
        "and",
        "or",
        "=",
        "(",
        ")",
        "{",
        "}",
        ",",
        ";",
      ];

      // Verifica si el stream ha llegado al final
      if (stream.eol()) {
        state.tokenize = null; // Restablece el tokenizador para el siguiente token
        return null; // Devuelve null para permitir que se continúe escribiendo
      }

      // Tokeniza números enteros y reales
      if (stream.match(/^\d*\.?\d+/)) {
        return "number"; // Usa una clase de estilo CSS "number" para números
      }

      // Tokeniza identificadores
      const identifier = stream.match(/^[_a-zA-Z][_a-zA-Z0-9]*/);
      if (identifier) {
        if (keywords.includes(identifier[0])) {
          return "keyword"; // Usa una clase de estilo CSS "keyword" para palabras reservadas
        } else {
          return "variable"; // Usa una clase de estilo CSS "variable" para identificadores
        }
      }

      // Tokeniza strings entre comillas dobles
      if (stream.match(/^\".*?\"/)) {
        return "string"; // Usa una clase de estilo CSS "string" para strings
      }
      // Tokeniza strings entre comillas simples
      if (stream.match(/^'.*?'/)) {
        return "string"; // Usa una clase de estilo CSS "string" para strings
      }

      // Tokeniza comentarios de una línea
      if (stream.match(/^\/\/.*/)) {
        return "comment"; // Usa una clase de estilo CSS "comment" para comentarios
      }

      // Tokeniza comentarios de múltiples líneas
      if (stream.match(/^\/\*.*\*\//)) {
        return "comment"; // Usa una clase de estilo CSS "comment" para comentarios
      }

      // Tokeniza operadores
      const operator = stream.match(/^[+\-*/%^<>=!&,;(){}]/);
      if (operator) {
        return "operator"; // Usa una clase de estilo CSS "operator" para operadores
      }

      // Avanza al siguiente token
      stream.next();
      return null; // Devuelve null para permitir que se continúe escribiendo
    },
  };
});

const cmOptions: EditorConfiguration = {
  mode: "customMode",
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
  console.log("Editor content changed:", val); // Agregar esta línea
  store.setContents(cm.getValue());
};

const onInput = (val: string) => {
  console.log("Editor input:", val); // Agregar esta línea
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

<style>
/* Estilos para resaltar números */
.cm-number {
  color: blue !important; /* Color azul para números */
}

/* Estilos para resaltar identificadores */
.cm-variable {
  color: green !important; /* Color verde para identificadores */
}

/* Estilos para resaltar comentarios */
.cm-comment {
  color: gray !important; /* Color gris para comentarios */
}

/* Estilos para resaltar palabras reservadas */
.cm-keyword {
  color: purple !important; /* Color morado para palabras reservadas */
}

/* Estilos para resaltar operadores */
.cm-operator {
  color: orange !important; /* Color naranja para operadores */
}
</style>
