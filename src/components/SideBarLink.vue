<template>
    <router-link :to="to" class="flex items-center gap-3 hover:bg-gray-600 active:bg-gray-400 p-3 no-underline h-8" :class="{ active: isActive }">
        <i class="shrink-0 w-10 color-white" :class="icon"></i>
        <span v-if="!collapsed" class="transition-opacity duration-300 opacity-100 hover:opacity-75 active:opacity-50 flex justify-center">
            <slot />
        </span>
    </router-link>
</template>



<script setup lang="ts">
import { computed, inject } from "vue";
import { useRoute } from "vue-router";
import { useStore } from "../stores/useStore";

const store = useStore();
let collapsed = computed(() => store.collapsed);
const route = useRoute();

interface MyProps {
  to: string;
  icon: string;
}

const props = defineProps<MyProps>();
console.log("props", props);
console.log(props.icon);
const isActive = computed(() => route.path === String(props.to));
</script>
