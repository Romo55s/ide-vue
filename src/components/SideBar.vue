<template>
  <div
    class="bg-neutral-950 text-white p-0.5 ease-out duration-100 flex flex-col h-full font-consolas"
    :style="{ width: sidebarWidth }"
  >
    <div class="flex justify-center mt-8 items-center">
      <img
        :src="collapsed ? luvenSingle : luven"
        alt="Logo"
        class="w-32 h-32"
      />
    </div>
    <SideBarLink
      to="/editor"
      icon="fa-regular fa-file"
      title="New File"
      @click="NewFile" class="small-text"
      >New file</SideBarLink
    >
    <SideBarLink
      to="/"
      icon="fa-regular fa-trash-can"
      title="Delete file"
      v-if="store.flagSave"
      @click="DeleteFile" class="small-text"
      >Delete file</SideBarLink
    >
    <SideBarLink
      to=""
      icon="fa-regular fa-folder-open"
      title="Open file"
      @click="readFileContents" class="small-text"
      >Open file</SideBarLink
    >
    <SideBarLink
      to=""
      icon="fa-regular fa-save"
      title="Save"
      @click="saveFileContents"
      v-if="!store.flagNewFile" class="small-text"
      >Save</SideBarLink
    >
    <SideBarLink
      to=""
      icon="fa-regular fa-pen-to-square"
      title="Save as"
      @click="saveAsFileContents" class="small-text"
      >Save as</SideBarLink
    >
    <SideBarLink to="" icon="fa-solid fa-play" title="Run" class="small-text">Run</SideBarLink>
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
import { computed, ref } from "vue";
import SideBarLink from "./SideBarLink.vue";
import luvenSingle from "../assets/icons/luven-single.svg";
import luven from "../assets/icons/luven.svg";
import { open, save } from "@tauri-apps/api/dialog";
import { useStore } from "../stores/useStore";
import { useRouter } from "vue-router";
import { BaseDirectory, readTextFile, writeTextFile } from "@tauri-apps/api/fs";
import { invoke } from "@tauri-apps/api/tauri";
import { toast } from "vue3-toastify";
import "vue3-toastify/dist/index.css";

const store = useStore();
const router = useRouter();
const contents = ref(store.contents);

const NewFile = async () => {
  store.setFlagSave(false);
  store.setFlagNewFile(true);
  await router.push("/editor");
  store.setContents("");
};

const DeleteFile = async () => {
  try {
    // Verificar si hay un archivo abierto para eliminar
    if (!store.paths) {
      toast.error("There is no file open to eliminate", {
        position: toast.POSITION.TOP_RIGHT,
        theme: "dark",
      });
      console.warn("No hay ningún archivo abierto para eliminar.");
      return;
    }

    // Eliminar el archivo utilizando invoke con el comando remove_file
    await invoke("remove_file", { path: store.paths });
    toast.success("File succesfuly deleted", {
      position: toast.POSITION.TOP_RIGHT,
      theme: "dark",
    });
    // Limpiar el contenido y el path del archivo
    store.setContents("");
    store.setPaths("");
    store.setFlagNewFile(true); // Si es necesario establecer alguna otra bandera, hazlo aquí
    store.setFlagEditor(false);
  } catch (error) {
    console.error("Error al intentar eliminar el archivo:", error);
  }
};

const saveFileContents = async () => {
  try {
    // Verifica si el contenido ha sido modificado
    if (!store.contents) {
      toast.error("There is no content to save", {
        position: toast.POSITION.TOP_RIGHT,
        theme: "dark",
      });
      console.warn("El contenido está vacío. No hay nada que guardar.");
      return;
    }

    // Verifica si hay un archivo abierto para sobrescribir
    if (!store.paths) {
      console.warn("No hay ningún archivo abierto para sobrescribir.");
      return;
    }

    // Sobrescribe el archivo existente con el contenido actual
    await writeTextFile(store.paths, store.contents);
    toast.success("File saved succesfuly", {
      position: toast.POSITION.TOP_RIGHT,
      theme: "dark",
    });
  } catch (error) {
    console.error("Error al intentar sobrescribir el archivo:", error);
  }
};

const saveAsFileContents = async () => {
  try {
    const result = await save();
    if (!result) {
      console.warn("No se seleccionó ningún archivo.");
      return;
    }
    store.setPaths(result);
    store.setFlagNewFile(false);
    store.setFlagSave(true);
    await invoke("save_file", { path: result, contents: store.contents });
  } catch (error) {
    console.error("Error al intentar guardar el archivo:", error);
  }
};

const readFileContents = async () => {
  try {
    const selectedPath = await open({
      multiple: false,
      title: "Open Text File",
    });
    if (!selectedPath) return;
    contents.value = await readTextFile(selectedPath as string);
    store.setContents(contents.value);
    store.setPaths(selectedPath as string);
    router.push("/editor");
    store.setFlagNewFile(false);
    store.setFlagSave(true);
  } catch (error) {
    console.log(error);
  }
};

const toggleSideBar = () => {
  store.toggleSidebar();
};

const sidebarWidth = computed(() => store.sidebarWidth);
const collapsed = computed(() => store.collapsed);
</script>
<style>
.small-text {
    font-size: 0.90rem;
  }
</style>