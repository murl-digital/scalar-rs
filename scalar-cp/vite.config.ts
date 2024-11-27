import { sveltekit } from "@sveltejs/kit/vite";
import { transformerDirectives } from "unocss";
import UnoCSS from "@unocss/svelte-scoped/vite";
import { defineConfig } from "vite";

export default defineConfig({
  plugins: [
    UnoCSS({
      cssFileTransformers: [transformerDirectives()],
    }),
    sveltekit(),
  ],
});
