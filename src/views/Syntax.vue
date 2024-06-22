<template>
  <div class="bg-neutral-950 min-h-full flex justify-center items-center text-white">
    <div class="max-w-3xl p-8">
      <h1 class="text-4xl font-bold mb-4">Syntax</h1>
      <div v-if="treeNodes.length > 0">
        <div class="card flex flex-col align-items-center">
          <div class="flex flex-wrap mb-6 space-x-2">
            <Button type="button" icon="pi pi-plus" label="Expand All" @click="expandAll" outlined />
            <Button type="button" icon="pi pi-minus" label="Collapse All" @click="collapseAll" outlined />
          </div>
          <Tree v-model:expandedKeys="expandedKeys" :value="treeNodes" class="w-full md:w-[30rem] custom-tree"></Tree>
        </div>
      </div>
      <div v-else>
        <p>No Syntactic tree available.</p>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from "vue";
import Tree from 'primevue/tree';
import Button from 'primevue/button';
import { invoke } from "@tauri-apps/api/tauri";
import { useStore } from "../stores/useStore";

interface TreeNode {
  key: string;
  label: string;
  children?: TreeNode[];
}

const store = useStore();
const tokens = ref(store.tokens);
const tree = ref<any>(null);
const errors = ref<string[] | null>(null);
const treeNodes = ref<TreeNode[]>([]);
const expandedKeys = ref<{ [key: string]: boolean }>({});

const generateSyntaxTree = async (): Promise<void> => {
  try {
    console.log("Calling parse with content:", tokens.value);
    const [result, errorsResult]: [any, string[]] = await invoke("parse", {
      tokens: tokens.value,
    });
    console.log("Received result:", result);
    console.log("Received errorsResult:", errorsResult);
    tree.value = result;
    treeNodes.value = [transformNode(result)];
    errors.value = errorsResult;
    store.resetErrors();
    store.setErrorsSyntax(errorsResult);
  } catch (error) {
    console.error("Error generating syntax tree:", error);
  }
};

const transformNode = (node: any): TreeNode => {
  const token = node.token || '-';
  const value = node.value || '-';
  const label = `${node.node_type} (token: ${token}, value: ${value})`;

  const transformedNode: TreeNode = {
    key: node.node_type,
    label,
    children: node.children?.map((child: any) => transformNode(child)) || [],
  };
  return transformedNode;
};

const expandAll = () => {
  if (treeNodes.value.length > 0) {
    for (let node of treeNodes.value) {
      expandNode(node);
    }
    expandedKeys.value = { ...expandedKeys.value };
  }
};

const collapseAll = () => {
  expandedKeys.value = {};
};

const expandNode = (node: TreeNode) => {
  expandedKeys.value[node.key] = true;
  if (node.children && node.children.length) {
    for (let child of node.children) {
      expandNode(child);
    }
  }
};

const redirectToAnotherView = (): void => {
  console.log("Redirecting to another view...");
};

onMounted(generateSyntaxTree);
</script>

