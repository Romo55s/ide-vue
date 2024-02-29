<script setup lang="ts">
import { RouterView } from "vue-router";
import  SideBar  from "@/components/SideBar.vue";
import { computed, inject } from 'vue';
import { useStore } from '@/stores/useStore';

const store = useStore();

let sidebarWidth = computed(() => store.sidebarWidth);
</script>

<template>
  <SideBar/>
  <div :style="{'margin-left': sidebarWidth as string}">
    <router-view></router-view>
  </div>
</template>

<style scoped></style>
