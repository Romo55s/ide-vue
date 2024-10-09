<template>
  <div class="bg-neutral-950 min-h-full flex justify-center items-center text-white">
    <div class="max-w-3xl p-8">
      <h1 class="text-4xl font-bold mb-4">Semantic</h1>
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
        <p>No semantic information available.</p>
      </div>
    </div>
  </div>
</template>

<script lang="ts" setup>
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
const semanticInfo = ref<string[]>([]);
const tree = ref<any>(null);
const treeNodes = ref<TreeNode[]>([]);
const expandedKeys = ref<{ [key: string]: boolean }>({});

const generateSemanticInfo = async () => {
  try {
    const syntaxTree = store.syntaxTree;
    if (!syntaxTree) {
      console.error("No syntax tree available");
      return;
    }

    const [result, errorsResult]: [any, string[]] = await invoke("analyze", {
      syntaxTree: syntaxTree,
    });

    store.setSemanticTree(result);
    store.setErrorsSemantic(errorsResult);

    // Actualiza semanticInfo con la información del resultado
    semanticInfo.value = result.semanticInfo || [];
    tree.value = result;
    treeNodes.value = [transformNode(result)];
  } catch (error) {
    console.error("Error generating semantic info:", error);
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

onMounted(generateSemanticInfo);
</script>

<style scoped>
.icon {
  cursor: pointer;
}

.muted {
  color: gray;
  font-size: 80%;
}
</style>