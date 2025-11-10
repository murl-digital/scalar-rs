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
    "border-base": [
      "border-none",
      "outline",
      "outline-1",
      "outline-gray",
      "rounded-sm",
      "ring",
      "ring-transparent",
      "ring-offset-2",
      "ring-offset-dark",
      "ring-2",
    ],
    "border-active": ["ring-purple"],
    "input-border": [
      "border-base",
      "hover:border-active",
      "focus-visible:border-active",
    ],
    "input-button": [
      "input-base",
      "hover:bg-neutral-600",
      "focus-visible:border-active",
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
