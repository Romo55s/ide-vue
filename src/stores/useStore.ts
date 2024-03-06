import { ref, computed } from "vue";
import { defineStore } from "pinia";

export const useStore = defineStore({
  id: "main",
  state: () => ({
    sidebarWidth: "10em",
    collapsed: false,
    contents: ref(""),
    flagNewFile: false,
  }),
  actions: {
    toggleSidebar() {
      this.collapsed = !this.collapsed;
      this.sidebarWidth = this.collapsed ? "4.5em" : "10em";
    },
    setContents(contents: string | null | undefined) {
      this.contents = contents || "";
    },
    setFlagNewFile(flag : boolean) {
      this.flagNewFile = flag;
    }
  },
});
