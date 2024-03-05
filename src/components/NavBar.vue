<template>
  <header class="bg-neutral-950 z-10 w-full relative text-white">
    <nav class="flex justify-start p-2 w-full mx-8 my-0 flex-row">
      <ul class="font-bold text-white text-xs flex space-x-4">
        <li
          class="relative"
          @mouseover="toggleDropdown('file', true)"
          @mouseleave="toggleDropdown('file', false)"
        >
          <a href="#" class="block px-4 py-2 hover:bg-neutral-800">File</a>
          <ul
            v-if="showDropdown.file"
            class="absolute top-full left-0 bg-neutral-700 text-white shadow-md"
          >
            <li>
              <a href="#" class="block px-4 py-2 hover:bg-neutral-800">Close</a>
            </li>
            <!-- More dropdown items for File here -->
          </ul>
        </li>
        <li
          class="relative"
          @mouseover="toggleDropdown('edit', true)"
          @mouseleave="toggleDropdown('edit', false)"
        >
          <a href="#" class="block px-4 py-2 hover:bg-neutral-800">Edit</a>
          <ul
            v-if="showDropdown.edit"
            class="absolute top-full left-0 bg-neutral-700 text-white shadow-md"
          >
            <li>
              <a href="#" class="block px-4 py-2 hover:bg-neutral-800">Copy</a>
            </li>
            <!-- More dropdown items for Edit here -->
          </ul>
        </li>
        <li
          class="relative"
          @mouseover="toggleDropdown('build', true)"
          @mouseleave="toggleDropdown('build', false)"
        >
          <a href="#" class="block px-4 py-2 hover:bg-neutral-800"
            >Build and Debug</a
          >
          <ul
            v-if="showDropdown.build"
            class="absolute top-full left-0 bg-neutral-700 text-white shadow-md"
          >
            <li>
                <a href="#" class="block px-4 py-2 hover:bg-neutral-800">Lexical</a>
            </li>
            <li>
              <a href="#" class="block px-4 py-2 hover:bg-neutral-800"
                >Syntax</a
              >
            </li>
            <li>
              <a href="#" class="block px-4 py-2 hover:bg-neutral-800"
                >Semantic</a
              >
            </li>
            <li>
              <a href="#" class="block px-4 py-2 hover:bg-neutral-800">Run</a>
            </li>
          </ul>
        </li>
      </ul>
    </nav>
  </header>
</template>

<script setup lang="ts">
import { ref } from "vue";

interface DropdownState {
  file: boolean;
  edit: boolean;
  build: boolean;
}

const showDropdown = ref<DropdownState>({
  file: false,
  edit: false,
  build: false,
});

const toggleDropdown = (dropdown: keyof DropdownState, show: boolean): void => {
  showDropdown.value[dropdown] = show;
};
</script>
