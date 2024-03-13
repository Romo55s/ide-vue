<script setup lang="ts">
import { RouterView } from "vue-router";
import SideBar from "./components/SideBar.vue";
import { useStore } from "./stores/useStore";
import NavBar from "./components/NavBar.vue";
import Terminal from "./components/Terminal.vue";
import { computed } from "vue";

const store = useStore();

const sidebarWidth = computed(() => store.sidebarWidth);
const collapsed = computed(() => store.collapsed);
</script>

<template>
  <div class="flex flex-col max-h-screen w-full">
    <NavBar/>
    <div class="flex">
      <div :style="{ width: sidebarWidth }">
        <SideBar />
      </div>
      <div class="flex-grow w-full h-full">
        <router-view></router-view>
      </div>
    </div>
  </div>
</template>

<style scoped>
</style>
