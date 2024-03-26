<template>
  <div class="bg-black min-h-full flex justify-start items-center text-green-400 flex-20">
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
            <tr v-for="(token, index) in tokens" :key="index" class="border-t border-green-400">
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
import { invoke } from '@tauri-apps/api/tauri';
import { ref, onMounted } from 'vue';

const tokens = ref<string[][]>([]);

const fetchTokens = async (content: string) => {
  try {
    const response = await invoke('lexic', { content });
    tokens.value = response;
  } catch (error) {
    console.error('Error fetching tokens:', error);
  }
};

onMounted(() => {
  // Aquí deberías obtener el contenido del archivo y pasarlo a la función fetchTokens
  const content = "..." // Obtener el contenido del archivo
  fetchTokens(content);
});
</script>

<style scoped>
/* Add any additional styling using Tailwind CSS classes */
</style>
