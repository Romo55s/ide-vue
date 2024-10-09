import { ref, computed } from "vue";
import { defineStore } from "pinia";

export const useStore = defineStore({
  id: "main",
  state: () => ({
    sidebarWidth: "10em",
    collapsed: false,
    contents: ref(""),
    paths: ref(""),
    flagNewFile: false,
    column: 0,
    row: 0,
    semanticTree: null, 
    syntaxTree: null,
    errorSemantic: [],
    flagEditor: false,
    flagSave: false,
    errors: [] as string[][],
    errorsSyntax: [] as string[],
    tokens: [] as string[][],
  }),
  actions: {
    toggleSidebar() {
      this.collapsed = !this.collapsed;
      this.sidebarWidth = this.collapsed ? "4.5em" : "10em";
    },
    setContents(contents: string | null | undefined) {
      this.contents = contents || "";
    },
    setPaths(paths: string | null | undefined) {
      this.paths = paths || "";
    },
    setFlagNewFile(flag: boolean) {
      this.flagNewFile = flag;
    },
    setColumn(column: number) {
      this.column = column + 1;
    },
    setRow(row: number) {
      this.row = row + 1;
    },
    setFlagEditor(flag: boolean) {
      this.flagEditor = flag;
    },
    setFlagSave(flag: boolean) {
      this.flagSave = flag;
    },
    setErrors(errors: string[][]) {
      this.errors = errors;
    },
    setTokens(tokens: string[][]) {
      this.tokens = tokens;
    },
    setErrorsSyntax(errors: string[]) {
      this.errorsSyntax = errors;
    },
    setSemanticTree(tree: any) {
      this.semanticTree = tree;
    },
    setErrorsSemantic(errors: string[]) {
      this.errorsSemantic = errors;
    },
    setSyntaxTree(tree: any) {
      this.syntaxTree = tree;
    },
    resetErrors() {
      this.errors = [];
      this.errorsSyntax = [];
      this.errorsSemantic = [];
    },
  },
});
