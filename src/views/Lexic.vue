<template>
  <div class="bg-neutral-900 min-h-full w-full flex justify-start items-center text-white flex-20 code-mirror-table">
    <div class="max-w-3xl p-8">
      <h1 class="text-4xl font-bold mb-4">Lexic</h1>
      <div v-if="tokens.length > 0">
        <table class="w-full text-left table-auto">
          <thead>
            <tr>
              <th class="px-4 py-2">Type</th>
              <th class="px-4 py-2">Lexeme</th>
              <th class="px-4 py-2">Line</th>
              <th class="px-4 py-2">Column</th>
            </tr>
          </thead>
          <tbody>
            <tr v-for="(token, index) in tokens" :key="index" :class="getTokenClass(token[0])">
              <td class="px-4 py-2">{{ token[0] }}</td>
              <td class="px-4 py-2">{{ token[1] }}</td>
              <td class="px-4 py-2">{{ token[2] }}</td>
              <td class="px-4 py-2">{{ token[3] }}</td>
            </tr>
          </tbody>
        </table>
      </div>
      <div v-else>
        <p>No tokens found.</p>
      </div>
    </div>
  </div>
</template>


<script setup lang="ts">
import { invoke } from "@tauri-apps/api/tauri";
import { ref, onMounted } from "vue";
import { useStore } from "../stores/useStore";

const store = useStore();
const tokens = ref<string[][]>(store.tokens);
const contents = ref<string>(store.contents);

const fetchTokens = async (content: string) => {
  try {
    const response = await invoke("lexic", { content: content });
    const [validTokens, errorTokens] = response as [string[][], string[][]];
    store.setTokens(validTokens);
    tokens.value = validTokens;
  } catch (error) {
    console.error("Error fetching tokens:", error);
  }
};

const getTokenClass = (tokenType: string) => {
  switch (tokenType) {
    case "number":
    case "NumInt":
    case "NumReal":
      return "token-number";
    case "ID":
      return "token-variable";
    case "InMultipleComment":
    case "InComment":
      return "token-comment";
    case "IF":
    case "ELSE":
    case "DO":
    case "WHILE":
    case "SWITCH":
    case "CASE":
    case "END":
    case "REPEAT":
    case "UNTIL":
    case "READ":
    case "WRITE":
    case "INTEGER":
    case "DOUBLE":
    case "MAIN":
    case "AND":
    case "OR":
    case "RETURN":
      return "token-keyword";
    case "PLUS":
    case "MINUS":
    case "TIMES":
    case "DIVIDE":
    case "MODULO":
    case "POWER":
    case "EQ":
    case "NEQ":
    case "LT":
    case "LTE":
    case "GT":
    case "GTE":
    case "LPAREN":
    case "RPAREN":
    case "LBRACE":
    case "RBRACE":
    case "COMMA":
    case "SEMICOLON":
    case "ASSIGN":
      return "token-operator";
    default:
      return "";
  }
};


onMounted(() => {
  fetchTokens(contents.value);
});
</script>



<style scoped>
.code-mirror-table {
  font-family: consolas !important;
}
/* Estilos para resaltar números */
.token-number {
  color: #0f80f1 !important; /* Azul Oscuro */
}

/* Estilos para resaltar identificadores */
.token-variable {
  color: #2b029b !important; /* Verde Oscuro */
}

/* Estilos para resaltar comentarios */
.token-comment {
  color: #333333 !important; /* Gris Oscuro */
}

/* Estilos para resaltar palabras reservadas */
.token-keyword {
  color: #f50097 !important; /* Morado */
}

/* Estilos para resaltar operadores */
.token-operator {
  color: #FFA500 !important; /* Naranja */
}
</style>