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
  <div class="max-h-screen w-full min-h-screen">
    <NavBar/>
    <div class="flex w-full h-screen">
      <div :style="{ width: sidebarWidth  }">
        <SideBar />
      </div>
      <div class="w-full h-full">
        <router-view></router-view>
      </div>
    </div>
  </div>
</template>

<style scoped>
</style>
