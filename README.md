# Luven

Luven is a Vue-based IDE template that aims to facilitate your development process.

## Recommended IDE Setup

- [VSCode](https://code.visualstudio.com/)
- [Volar](https://marketplace.visualstudio.com/items?itemName=Vue.volar) (and disable Vetur)
- [TypeScript Vue Plugin (Volar)](https://marketplace.visualstudio.com/items?itemName=Vue.vscode-typescript-vue-plugin)

## Type Support for `.vue` Imports in TS

TypeScript cannot handle type information for `.vue` imports by default, so we replace the `tsc` CLI with `vue-tsc` for type checking. In editors, we need [TypeScript Vue Plugin (Volar)](https://marketplace.visualstudio.com/items?itemName=Vue.vscode-typescript-vue-plugin) to make the TypeScript language service aware of `.vue` types.

If the standalone TypeScript plugin doesn't feel fast enough to you, Volar has also implemented a [Take Over Mode](https://github.com/johnsoncodehk/volar/discussions/471#discussioncomment-1361669) that is more performant. You can enable it by following these steps:

1. Disable the built-in TypeScript Extension
    - Run `Extensions: Show Built-in Extensions` from VSCode's command palette
    - Find `TypeScript and JavaScript Language Features`, right click and select `Disable (Workspace)`
2. Reload the VSCode window by running `Developer: Reload Window` from the command palette.

## Customize configuration

See [Vite Configuration Reference](https://vitejs.dev/config/).

## Project Setup

To run the program natively, follow these steps:

1. Download the code and navigate to the project directory.
2. Run `npm install` to download the necessary dependencies.
3. Ensure that Rust is installed, as we are using Tauri to compile our application into Rust. You can install Rust from [here](https://www.rust-lang.org/tools/install).
4. Run the command `npx tauri dev` or `cargo tauri dev` to run the application.

### Compile and Hot-Reload for Development

To compile and hot-reload the project for development, use the following command:

```sh
npm run dev
```

To lint the code and check for any style inconsistencies, use the following command:
```sh

npm run lint
```

To build the project for production, use the following command:
```sh

npm run build
```