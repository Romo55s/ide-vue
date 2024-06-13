<template>
  <div
    class="bg-neutral-950 min-h-full flex justify-center items-center text-white flex-20 font-consolas"
  >
    <div class="max-w-3xl p-8">
      <h1 class="text-4xl font-bold mb-4">Errors</h1>
      <div v-if="store.errors.length > 0">
        <table class="w-full table-auto">
          <thead>
            <tr>
              <th class="px-4 py-2">Type</th>
              <th class="px-4 py-2">Lexeme</th>
              <th class="px-4 py-2">Line</th>
              <th class="px-4 py-2">Column</th>
            </tr>
          </thead>
          <tbody>
            <tr
              v-for="(token, index) in store.errors"
              :key="index"
              class="border-t border-green-400"
            >
              <td class="px-4 py-2">{{ token[0] }}</td>
              <td class="px-4 py-2">{{ token[1] }}</td>
              <td class="px-4 py-2">{{ token[2] }}</td>
              <td class="px-4 py-2">{{ token[3] }}</td>
            </tr>
          </tbody>
        </table>
      </div>
      <div v-if="store.errorsSyntax.length > 0">
        <ul>
          <li v-for="(error, index) in store.errorsSyntax" :key="index">{{ error }}</li>
        </ul>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { invoke } from "@tauri-apps/api/tauri";
import { ref, onMounted } from "vue";
import { useStore } from "../stores/useStore";

const store = useStore();
const contents = ref(store.contents);

console.log(store.errors);
console.log(store.errorsSyntax);

onMounted(() => {});
</script>

<style scoped></style>
