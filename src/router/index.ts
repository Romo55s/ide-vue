import { createRouter, createWebHistory } from "vue-router";
import Editor from "../components/Editor.vue";
import Welcome from "../views/Welcome.vue";

import Analizer from "../views/Analizers.vue";
import Errors from "../views/Errors.vue";
import Lexic from "../views/Lexic.vue";
import Semantic from "../views/Semantic.vue";
import Syntax from "../views/Syntax.vue";
import Run from "../views/Run.vue";

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: "/",
      name: "home",
      component: Welcome,
    },
    {
      path: "/editor",
      name: "editor",
      component: Editor,
      children: [
        {
          path: "/analizer",
          name: "analizer",
          component: Analizer,
          children: [
            {
              path: "errors",
              name: "errors",
              component: Errors,
            },
            {
              path: "lexic",
              name: "lexic",
              component: Lexic,
            },
            {
              path: "semantic",
              name: "semantic",
              component: Semantic,
            },
            {
              path: "syntax",
              name: "syntax",
              component: Syntax,
            },
            {
              path: "run",
              name: "run",
              component: Run,
            },
          ],
        },
      ],
    },
  ],
});

export default router;
