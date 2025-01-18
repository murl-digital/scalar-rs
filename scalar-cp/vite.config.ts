import { sveltekit } from "@sveltejs/kit/vite";
import { transformerDirectives } from "unocss";
import UnoCSS from "@unocss/svelte-scoped/vite";
import { defineConfig } from "vite";
import { splashScreen } from "vite-plugin-splash-screen";

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
    splashScreen({
      logoSrc: "logo.svg",
      splashBg: "#222",
    }),
    UnoCSS({
      cssFileTransformers: [transformerDirectives()],
    }),
    sveltekit(),
  ],
  define: {
    __ENABLE_CARTA_SSR_HIGHLIGHTER__: false,
  },
});
