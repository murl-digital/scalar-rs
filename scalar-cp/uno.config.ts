import { defineConfig, presetIcons, presetUno } from "unocss";
import { presetForms } from "@julr/unocss-preset-forms";

export default defineConfig({
  presets: [presetUno(), presetForms(), presetIcons()],
  theme: {
    animation: {
      durations: {
        pulse: "5s",
      },
    },
  },
});
