<template>
  <div
    class="bg-neutral-950 min-h-full flex justify-center items-center text-white"
  >
    <div class="max-w-3xl p-8">
      <h1 class="text-4xl font-bold mb-4">Syntactic</h1>
      <div v-if="json">
        <JsonTreeView :json="json" :maxDepth="3" />
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

const store = useStore();
const contents = ref(store.contents);
console.log(contents.value);
const json = ref<string | null>(null);

const generateSyntaxTree = async (): Promise<void> => {
  try {
    const result = await invoke("parse", {
      content: contents.value,
    });
    json.value = result as string;
  } catch (error) {
    console.error("Error generating syntax tree:", error);
  }
};

const redirectToAnotherView = (): void => {
  console.log("Redirigiendo a otra vista...");
};

const handleClick = (): void => {
  generateSyntaxTree();
  redirectToAnotherView();
};

onMounted(generateSyntaxTree);
</script>