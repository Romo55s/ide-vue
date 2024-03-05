<script setup lang="ts">
import { RouterView } from "vue-router";
import SideBar from "./components/SideBar.vue";
import { useStore } from './stores/useStore';
import NavBar from "./components/NavBar.vue";
import Terminal from "./components/Terminal.vue";
import { computed } from "vue";

const store = useStore();

let sidebarWidth = computed(() => store.sidebarWidth);
</script>

<template>
  <NavBar/>
  <SideBar/>
  <div :style="{'margin-left': sidebarWidth as string}">
    <router-view></router-view>
  </div>
  <!-- <Terminal/> -->
</template>

<style scoped></style>
