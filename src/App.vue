<script setup lang="ts">
import { RouterView } from "vue-router";
import SideBar from "./components/SideBar.vue";
import { useStore } from "./stores/useStore";
import NavBar from "./components/NavBar.vue";
import Terminal from "./components/Terminal.vue";
import { computed } from "vue";

const store = useStore();

let sidebarWidth = computed(() => store.sidebarWidth);
</script>

<template>
  <div class="overflow-hidden w-full h-full">
    <NavBar />
    <SideBar />
    <div>
      <router-view></router-view>
    </div>
  </div>
  <!-- <Terminal/> -->
</template>

<style scoped></style>
