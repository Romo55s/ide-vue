<template>
    <div
        class="bg-gray-800 float-left text-white fixed z-1 p-0.5 ease-out duration-100 flex flex-col h-full"
        :style="{ width: sidebarWidth }"
    >
        <div class="flex justify-center mt-8">
            <img :src="collapsed ? luvenSingle : luven" alt="Logo" class="w-32 h-32"/>
        </div>
        <SideBarLink to="/" icon="fa-regular fa-file" @click="readFileContents">File</SideBarLink>
        <SideBarLink to="/about" icon="fa-solid fa-gear">Settings</SideBarLink>
        <span
            class="absolute bottom-0 p-4 color-white ease-linear duration-200"
            @click="toggleSideBar"
            :class="{ 'transform rotate-180': collapsed }"
        >
            <i class="fas fa-angle-double-left"></i>
           
        </span>
    </div>
</template>

<script setup lang="ts">
import { ref, computed, provide } from "vue";
import SideBarLink from "./SideBarLink.vue";
import luvenSingle from '../assets/icons/luven-single.svg';
import luven from '../assets/icons/luven.svg';
import { useStore } from '../stores/useStore';
import {open} from "@tauri-apps/api/dialog";

const readFileContents = async () =>{
    try {
        const selectedPath = await open({
            defaultPath: ".",
            title: "Select a file",
            multiple: false,
            directory: false,
        });
        console.log(selectedPath);
    } catch (error) {
        console.log(error);
    }
}

const store = useStore();

let toggleSideBar = () => {
  store.toggleSidebar();
};

let sidebarWidth = computed(() => store.sidebarWidth);
let collapsed = computed(() => store.collapsed);

</script>
