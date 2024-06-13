<template>
  <div
    class="bg-neutral-950 min-h-full flex justify-center items-center text-white"
  >
    <div class="max-w-3xl p-8">
      <h1 class="text-4xl font-bold mb-4">Syntax</h1>
      <div v-if="json">
        <!-- Aquí deberías renderizar tu árbol sintáctico, esto es solo un ejemplo -->
        <json-tree-view :json="JSON.stringify(json)" :colorScheme="colorScheme" />
      </div>
      <div v-else>
        <p>No Syntactic tree available.</p>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from "vue";
import { JsonTreeView } from "json-tree-view-vue3";
import "json-tree-view-vue3/dist/style.css";
import { invoke } from "@tauri-apps/api/tauri";
import { useStore } from "../stores/useStore";


interface TreeNode {
  // Define las propiedades de TreeNode aquí
  // Por ejemplo:
  type: string;
  children: TreeNode[];
  // etc...
}

const colorScheme = 'dark';
const store = useStore();
const contents = ref(store.contents);
console.log(contents.value);
const json = ref<TreeNode | null>(null);
const errors = ref<string[] | null>(null);

const generateSyntaxTree = async (): Promise<void> => {
  try {
    console.log("Calling parse with content:", contents.value); // Imprimir el contenido
    const [result, errorsResult]: [TreeNode, string[]] = await invoke("parse", {
      content: contents.value,
    });
    console.log("Received result:", result); // Imprimir el resultado
    console.log("Received errorsResult:", errorsResult); // Imprimir los errores
    json.value = result;
    errors.value = errorsResult;
    store.resetErrors();
    store.setErrorsSyntax(errorsResult);
  } catch (error) {
    console.error("Error generating syntax tree:", error);
  }
};

const redirectToAnotherView = (): void => {
  console.log("Redirigiendo a otra vista...");
};

onMounted(generateSyntaxTree);
</script>

<style scoped>
.tree-view-item-key {
    color: red;
}
</style>