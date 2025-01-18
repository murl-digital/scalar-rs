import adapter from "@sveltejs/adapter-static";
import { preprocessMeltUI, sequence } from "@melt-ui/pp";
import { vitePreprocess } from "@sveltejs/vite-plugin-svelte";
import UnoCSS from "@unocss/svelte-scoped/preprocess";
const prod = process.env.NODE_ENV !== "development";
/** @type {import('@sveltejs/kit').Config}*/
const config = {
  // Consult https://kit.svelte.dev/docs/integrations#preprocessors
  // for more information about preprocessors
  preprocess: [
    vitePreprocess(),
    UnoCSS({
      combine: prod,
    }),
    preprocessMeltUI(),
  ],
  kit: {
    // adapter-auto only supports some environments, see https://kit.svelte.dev/docs/adapter-auto for a list.
    // If your environment is not supported, or you settled on a specific environment, switch out the adapter.
    // See https://kit.svelte.dev/docs/adapters for more information about adapters.
    //adapter: adapter(),
    alias: {
      "$ts/*": "../typescript_bindings/",
    },

    adapter: adapter({
      fallback: "index.html",
    }),
  },
  output: {
    bundleStrategy: "inline",
  },
};
export default config;
