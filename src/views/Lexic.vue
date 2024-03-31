<template>
<div class="bg-neutral-950 min-h-full w-full flex justify-center items-center text-white flex-20">
  <div class="max-w-3xl p-8">
    <h1 class="text-4xl font-bold mb-4 font-consolas">Lexic</h1>
    <div v-if="tokens.length > 0">
      <table class="w-full table-auto font-consolas">
        <thead>
          <tr>
            <th class="px-4 py-2">Type</th>
            <th class="px-4 py-2">Lexeme</th>
            <th class="px-4 py-2">Line</th>
            <th class="px-4 py-2">Column</th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="(token, index) in tokens" :key="index"  class="border-t border-green-400">
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
const error = ref<string[][]>([]);
const contents = ref<string>(store.contents);

const fetchTokens = async (content: string) => {
  try {
    const response = await invoke("lexic", { content: content });
    const [validTokens, errorTokens] = response as [string[][], string[][]];
    store.setTokens(validTokens);
    tokens.value = validTokens;
    error.value = errorTokens;
    store.setErrors(errorTokens);
    console.log(tokens.value);
    console.log(error.value);
  } catch (error) {
    console.error("Error fetching tokens:", error);
  }
};

onMounted(() => {
  fetchTokens(contents.value);
});
</script>
