import {
  defineConfig,
  presetIcons,
  presetTypography,
  presetUno,
  transformerDirectives,
} from "unocss";
import { presetForms } from "@julr/unocss-preset-forms";

export default defineConfig({
  presets: [presetUno(), presetForms(), presetIcons(), presetTypography()],
  transformers: [transformerDirectives()],
  shortcuts: {
    "input-base": [
      "input-border",
      "bg-neutral-700",
      "text-white",
      "transition-all",
    ],
    "input-border": [
      "border-none",
      "outline",
      "outline-1",
      "outline-gray",
      "rounded-sm",
      "ring",
      "ring-transparent",
      "hover:ring-purple",
      "focus:ring-purple",
      "focus-visible:ring-purple",
      "ring-offset-2",
      "ring-offset-dark",
      "ring-2",
    ],
    "input-button": [
      "input-base",
      "hover:bg-neutral-600",
      "m-2",
      "px-2",
      "py-1",
    ],
  },
  theme: {
    animation: {
      durations: {
        pulse: "5s",
      },
    },
  },
});
