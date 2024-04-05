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
import { ref } from "vue";
import { useStore } from "../stores/useStore";

const store = useStore();
const tokens = ref<string[][]>(store.tokens);
</script>
