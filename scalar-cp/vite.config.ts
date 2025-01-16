import { sveltekit } from "@sveltejs/kit/vite";
import { transformerDirectives } from "unocss";
import UnoCSS from "@unocss/svelte-scoped/vite";
import { defineConfig } from "vite";

export default defineConfig({
  server: {
    proxy: {
      "/api": {
        target: "http://localhost:3000",
        rewrite: (path) => path.replace(/^\/api/, ""),
      },
    },
  },
  plugins: [
    UnoCSS({
      cssFileTransformers: [transformerDirectives()],
    }),
    sveltekit(),
  ],
  define: {
    __ENABLE_CARTA_SSR_HIGHLIGHTER__: false,
  },
});
