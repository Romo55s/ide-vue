<template>
  <header class="bg-neutral-950 z-50 w-full flex-0 text-white sticky h-12 font-consolas">
    <nav class="flex justify-between p-2 w-full flex-row">
      <ul class="text-white text-xs flex space-x-4">
        <li
          class="relative"
          @mouseover="toggleDropdown('file', true)"
          @mouseleave="toggleDropdown('file', false)"
        >
          <a href="#" class="block px-4 py-2 hover:bg-neutral-800">File</a>
          <ul
            v-if="showDropdown.file"
            class="absolute top-auto left-0 bg-neutral-700 text-white shadow-md w-28"
          >
            <li>
              <a
                href="#"
                class="block px-4 py-2 hover:bg-neutral-800"
                @click="NewFile"
                >New file</a
              >
            </li>
            <li>
              <a
                href="#"
                class="block px-4 py-2 hover:bg-neutral-800"
                @click="readFileContents"
                >Open file</a
              >
            </li>
            <li>
              <a
                href="#"
                class="block px-4 py-2 hover:bg-neutral-800"
                @click="saveFileContents"
                v-if="!store.flagNewFile"
                >Save</a
              >
            </li>
            <li>
              <a
                href="#"
                class="block px-4 py-2 hover:bg-neutral-800"
                @click="saveAsFileContents"
                >Save as</a
              >
            </li>
            <li>
              <a
                href="#"
                class="block px-4 py-2 hover:bg-neutral-800"
                @click="closeFile"
                >Close</a
              >
            </li>
            <li>
              <a
                href="#"
                class="block px-4 py-2 hover:bg-neutral-800"
                @click="DeleteFile"
                >Delete file</a
              >
            </li>
          </ul>
        </li>
        <li
          class="relative"
          @mouseover="toggleDropdown('edit', true)"
          @mouseleave="toggleDropdown('edit', false)"
        >
          <a href="#" class="block px-4 py-2 hover:bg-neutral-800">Edit</a>
          <ul
            v-if="showDropdown.edit"
            class="absolute top-auto left-0 bg-neutral-700 text-white shadow-md w-28"
          >
            <li>
              <a href="#" class="block px-4 py-2 hover:bg-neutral-800">Copy</a>
            </li>
          </ul>
        </li>
        <li>
          <a href="/run" class="block px-4 py-2 hover:bg-neutral-800">Run</a>
        </li>
      </ul>
      <li
        v-if="store.flagEditor"
        class="flex gap-4 mr-12 font-bold text-white text-xs space-x-4 justify-center items-center"
      >
        <p>Ln: {{ store.row }}</p>
        <p>Col: {{ store.column }}</p>
      </li>
    </nav>
  </header>
</template>

<script setup lang="ts">
import { ref } from "vue";
import { useStore } from "../stores/useStore";
import { useRouter } from "vue-router";
import { invoke } from "@tauri-apps/api";
import { readTextFile, writeTextFile } from "@tauri-apps/api/fs";
import { open, save } from "@tauri-apps/api/dialog";
import { toast } from "vue3-toastify";
import "vue3-toastify/dist/index.css";

const store = useStore();
const router = useRouter();
const contents = ref(store.contents);
const paths = ref(store.paths);

const closeFile = () => {
  store.setContents("");
  store.setFlagEditor(false);
  store.setFlagSave(false);
  router.push("/");
};

const NewFile = async () => {
  store.setFlagNewFile(true);
  store.setFlagSave(false);
  await router.push("/editor");
  store.setContents("");
};

const DeleteFile = async () => {
  try {
    // Verificar si hay un archivo abierto para eliminar
    if (!store.paths) {
      console.warn("No hay ningún archivo abierto para eliminar.");
      return;
    }

    // Eliminar el archivo utilizando invoke con el comando remove_file
    await invoke("remove_file", { path: store.paths });
    // Limpiar el contenido y el path del archivo
    store.setContents("");
    store.setPaths("");
    store.setFlagNewFile(true); // Si es necesario establecer alguna otra bandera, hazlo aquí
    router.push("/");
    store.setFlagEditor(false);
    toast.success("File succesfuly deleted", {
      position: toast.POSITION.TOP_RIGHT,
      theme: "dark",
    });
  } catch (error) {
    console.error("Error al intentar eliminar el archivo:", error);
  }
};

const saveFileContents = async () => {
  try {
    // Verifica si el contenido ha sido modificado
    if (!store.contents) {
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
      return;
    }
    store.setPaths(result);
    store.setFlagNewFile(false);
    await invoke("save_file", { path: result, contents: store.contents });
    store.setFlagSave(true);
  } catch (error) {
    console.error("Error while trying to save the file:", error);
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

interface DropdownState {
  file: boolean;
  edit: boolean;
  build: boolean;
}

const showDropdown = ref<DropdownState>({
  file: false,
  edit: false,
  build: false,
});

const toggleDropdown = (dropdown: keyof DropdownState, show: boolean): void => {
  showDropdown.value[dropdown] = show;
};
</script>
