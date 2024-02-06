# frontend

This template should help get you started developing with Vue 3 in Vite.

## Recommended IDE Setup

[VSCode](https://code.visualstudio.com/) + [Volar](https://marketplace.visualstudio.com/items?itemName=Vue.volar) (and disable Vetur) + [TypeScript Vue Plugin (Volar)](https://marketplace.visualstudio.com/items?itemName=Vue.vscode-typescript-vue-plugin).

## Type Support for `.vue` Imports in TS

TypeScript cannot handle type information for `.vue` imports by default, so we replace the `tsc` CLI with `vue-tsc` for type checking. In editors, we need [TypeScript Vue Plugin (Volar)](https://marketplace.visualstudio.com/items?itemName=Vue.vscode-typescript-vue-plugin) to make the TypeScript language service aware of `.vue` types.

If the standalone TypeScript plugin doesn't feel fast enough to you, Volar has also implemented a [Take Over Mode](https://github.com/johnsoncodehk/volar/discussions/471#discussioncomment-1361669) that is more performant. You can enable it by the following steps:

1. Disable the built-in TypeScript Extension
    1) Run `Extensions: Show Built-in Extensions` from VSCode's command palette
    2) Find `TypeScript and JavaScript Language Features`, right click and select `Disable (Workspace)`
2. Reload the VSCode window by running `Developer: Reload Window` from the command palette.

## Customize configuration

See [Vite Configuration Reference](https://vitejs.dev/config/).

## Project Setup

```sh
npm install
```

### Compile and Hot-Reload for Development

```sh
npm run dev
```

### Type-Check, Compile and Minify for Production

```sh
npm run build
```

### Lint with [ESLint](https://eslint.org/)

```sh
npm run lint
```

# Tailwind Setup

[Source page](https://tailwindcss.com/docs/installation)

## Install Tailwind CSS

Install `tailwindcss` via npm, and create your `tailwind.config.js` file.

```
npm install -D tailwindcss
npx tailwindcss init
```

## Configure your template paths (Unverified)

Add the paths to all of your template files in your `tailwind.config.js` file.

```
/** @type {import('tailwindcss').Config} */
export default {
  content: ["./src/**/*.{html,js,ts,vue}"],
  theme: {
    extend: {},
  },
  plugins: [],
}
```

## Add the Tailwind directives to your CSS

Add the `@tailwind` directives for each of Tailwind's layers to your main CSS file. (`src/assets/main.css`)

```
@tailwind base;
@tailwind components;
@tailwind utilities:
```

## Start the Tailwind CLI build process

Run the CLI tool to scan your template files for classes and build your CSS.
Notice as this is a continuous process, and will consume your terminal session.

```
npx tailwindcss -i ./src/assets/main.css -o ./src/assets/output.css --watch
```