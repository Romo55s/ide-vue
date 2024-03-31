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
        class="dark-theme"
      />
      <div class="font-consolas">
        <nav
          class="flex min-w-full bg-neutral-950 text-white w-full overflow-y-auto"
        >
          <ul class="flex w-full">
            <!-- Agrega la clase w-full para que los botones abarquen todo el ancho -->
            <!-- Botones para recargar rutas -->
            <li v-if="store.errors.length > 0" class="flex-1 border border-white p-2">
              <!-- Utiliza la clase flex-1 para que los elementos se expandan -->
              <button
                @click="reloadRoute('/analizer/errors')"
                class="hover:text-gray-300 flex flex-col items-center w-full"
              >
                <i class="fas fa-triangle-exclamation"></i>
                <!-- Icono de error -->
                Errors
              </button>
            </li>
            <li class="flex-1 border border-white p-2">
              <!-- Utiliza la clase flex-1 para que los elementos se expandan -->
              <button
                @click="reloadRoute('/analizer/lexic')"
                class="hover:text-gray-300 flex flex-col items-center w-full"
              >
                <i class="fas fa-language"></i>
                <!-- Icono de léxico -->
                Lexical
              </button>
            </li>
            <li class="flex-1 border border-white p-2">
              <!-- Utiliza la clase flex-1 para que los elementos se expandan -->
              <button
                @click="reloadRoute('/analizer/syntax')"
                class="hover:text-gray-300 flex flex-col items-center w-full"
              >
                <i class="fas fa-puzzle-piece"></i>
                <!-- Icono de sintaxis -->
                Syntax
              </button>
            </li>
            <li class="flex-1 border border-white p-2">
              <!-- Utiliza la clase flex-1 para que los elementos se expandan -->
              <button
                @click="reloadRoute('/analizer/semantic')"
                class="hover:text-gray-300 flex flex-col items-center w-full"
              >
                <i class="fas fa-atom"></i>
                <!-- Icono de semántica -->
                Semantic
              </button>
            </li>
          </ul>
        </nav>

        <div class="w-full h-full overflow-y-auto overflow-x-auto">
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
import Codemirror from "codemirror-editor-vue3";
import type { CmComponentRef } from "codemirror-editor-vue3";
import type { Editor, EditorConfiguration } from "codemirror";
import "codemirror/lib/codemirror.css";

import { useStore } from "../stores/useStore";
import Analizers from "../views/Analizers.vue";
import { defineMode } from "codemirror";
import { useRouter } from "vue-router";

// Obtenemos el enrutador
const router = useRouter();

// Función para recargar la ruta actual
const reloadRoute = (path) => {
  // Obtenemos la ruta actual
  const currentRoute = router.currentRoute.value.path;

  // Verificamos si la ruta actual es igual a la ruta que se intenta recargar
  if (currentRoute === path) {
    // Forzamos la recarga de la ruta actual
    router.replace({ path: path });
  } else {
    // Redirigimos a la ruta especificada
    router.push({ path: path });
  }
};
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
.dark-theme .CodeMirror {
  background-color: #000000; /* Color de fondo oscuro */
}
/* Estilos para resaltar números */
.cm-number {
  color: #0f80f1 !important; /* Azul */
}

/* Estilos para resaltar identificadores */
.cm-variable {
  color: #6a42d7 !important; /* Morado */
}

/* Estilos para resaltar comentarios */
.cm-comment {
  color: #333333 !important; /* Gris Oscuro */
}

/* Estilos para resaltar palabras reservadas */
.cm-keyword {
  color: #f50097 !important; /* Rosa */
}

/* Estilos para resaltar operadores */
.cm-operator {
  color: #ffa500 !important; /* Naranja */
}
</style>
