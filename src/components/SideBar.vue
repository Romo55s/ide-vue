<template>
    <div class="bg-gray-800 float-left text-white fixed z-1 p-0.5 ease-out duration-100 flex flex-col h-full"
        :style="{ width: sidebarWidth }">
        <div class="flex justify-center mt-8">
            <img :src="collapsed ? luvenSingle : luven" alt="Logo" class="w-32 h-32" />
        </div>
        <SideBarLink to="/editor" icon="fa-regular fa-file" title="New File" style="color: #FFC200;">New file</SideBarLink>
        <SideBarLink to="/" icon="fa-regular fa-folder-open" title="Open file" style="color: rgb(3,172,240);" @click="readFileContents">Open file</SideBarLink>
        <SideBarLink to="/" icon="fa-regular fa-save" title="Save" style="color: #00AE6B;">Save</SideBarLink>
        <SideBarLink to="/" icon="fa-regular fa-edit" title="Save as" style="color: #875AF8;">Save as</SideBarLink>

        <span class="absolute bottom-0 p-4 color-white ease-linear duration-200" @click="toggleSideBar"
            :class="{ 'transform rotate-180': collapsed }">
            <i class="fas fa-angle-double-left"></i>

        </span>
    </div>
</template>

<script setup lang="ts">
import { computed, ref } from "vue";
import SideBarLink from "./SideBarLink.vue";
import luvenSingle from '../assets/icons/luven-single.svg';
import luven from '../assets/icons/luven.svg';
import { open } from "@tauri-apps/api/dialog";
import { useStore } from '../stores/useStore';
import { useRouter } from 'vue-router';
import { readTextFile } from "@tauri-apps/api/fs";

const store = useStore();
const router = useRouter();
const contents = ref("");

const readFileContents = async () => {
    try {
        const selectedPath = await open({
            multiple: false,
            title: 'Open Text File'
        });
        if (!selectedPath) return;
        contents.value = await readTextFile(selectedPath as string);
        console.log(contents.value);
        store.setContents(contents.value);
        router.push('/editor');
    } catch (error) {
        console.log(error);
    }
}

const toggleSideBar = () => {
    store.toggleSidebar();
};

const sidebarWidth = computed(() => store.sidebarWidth);
const collapsed = computed(() => store.collapsed);
</script>
