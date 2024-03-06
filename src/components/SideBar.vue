<template>
    <div class="bg-neutral-950 float-left text-white relative z-1 p-0.5 ease-out duration-100 flex flex-col min-h-[100vh] max-h-min"
        :style="{ width: sidebarWidth }">
        <div class="flex justify-center mt-8 items-center">
            <img :src="collapsed ? luvenSingle : luven" alt="Logo" class="w-32 h-32" />
        </div>
        <SideBarLink to="/editor" icon="fa-regular fa-file" title="New File" @click="NewFile">New file</SideBarLink>
        <SideBarLink to="/" icon="fa-regular fa-folder-open" title="Open file" @click="readFileContents">Open file</SideBarLink>
        <SideBarLink to="/" icon="fa-regular fa-save" title="Save" @click="saveFileContents" v-if="!store.flagNewFile">Save</SideBarLink>
        <SideBarLink to="/" icon="fa-solid fa-play" title="Run" >Run</SideBarLink>
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
import { open, save } from "@tauri-apps/api/dialog";
import { useStore } from '../stores/useStore';
import { useRouter } from 'vue-router';
import { BaseDirectory, readTextFile, writeTextFile } from "@tauri-apps/api/fs";
import { invoke } from '@tauri-apps/api/tauri'

const store = useStore();
const router = useRouter();
const contents = ref(store.contents);
console.log('Contents from store:', contents.value);

const NewFile = async () => {
    store.setFlagNewFile(true);
    await router.push('/editor');
    store.setContents("");
};


const saveFileContents = async () => {
    try {
        const result = await save();
        if (!result) {
            console.warn("No se seleccionó ningún archivo.");
            return;
        }
        console.log(result);
        console.log(store.contents);

        await invoke("save_file", { path: result, contents: store.contents });
    } catch (error) {
        console.error("Error al intentar guardar el archivo:", error);
    }
}



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
        store.setFlagNewFile(false);
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
