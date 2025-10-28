import type { EditorField } from "$ts/EditorField";
import type { EditorType } from "$ts/EditorType";
import type { Component } from "svelte";
import type { Errors } from "./types";

export type FieldComponent = Component<
  {
    field: EditorField;
    data: any;
    ready: () => void;
    errors?: Errors;
  },
  {},
  "data"
>;

export type ComponentMeta = {
  component: FieldComponent;
};

const components: Map<string, () => Promise<ComponentMeta>> = new Map([
  [
    "toggle",
    async () => {
      return {
        component: (await import("./components/types/ToggleInput.svelte"))
          .default,
      };
    },
  ],
  [
    "enum",
    async () => {
      return {
        component: (await import("./components/types/EnumDropdown.svelte"))
          .default,
      };
    },
  ],
  [
    "bool",
    async () => {
      return {
        component: (await import("./components/types/BoolInput.svelte"))
          .default,
      };
    },
  ],
  [
    "integer",
    async () => {
      return {
        component: (await import("./components/types/IntegerInput.svelte"))
          .default,
      };
    },
  ],
  [
    "float",
    async () => {
      return {
        component: (await import("./components/types/FloatInput.svelte"))
          .default,
      };
    },
  ],
  [
    "single-line",
    async () => {
      return {
        component: (await import("./components/types/SingleLineInput.svelte"))
          .default,
      };
    },
  ],
  [
    "multi-line",
    async () => {
      return {
        component: (await import("./components/types/MultiLineInput.svelte"))
          .default,
      };
    },
  ],
  [
    "markdown",
    async () => {
      return {
        component: (await import("./components/types/MarkdownInput.svelte"))
          .default,
      };
    },
  ],
  [
    "date",
    async () => {
      return {
        component: (await import("./components/types/DateTimeInput.svelte"))
          .default,
      };
    },
  ],
  [
    "date-time",
    async () => {
      return {
        component: (await import("./components/types/DateTimeInput.svelte"))
          .default,
      };
    },
  ],
  [
    "array",
    async () => {
      return {
        component: (await import("./components/types/ArrayInput.svelte"))
          .default,
      };
    },
  ],
  [
    "color-input",
    async () => {
      return {
        component: (await import("./components/types/ColorInput.svelte"))
          .default,
      };
    },
  ],
  [
    "image",
    async () => {
      return {
        component: (await import("./components/types/ImageInput.svelte"))
          .default,
      };
    },
  ],
  [
    "struct",
    async () => {
      return {
        component: (await import("./components/types/StructInput.svelte"))
          .default,
      };
    },
  ],
]);

export async function getComponent(
  type: EditorType,
): Promise<ComponentMeta | null> {
  console.log(type.component_key);
  let create = components.get(type.component_key ?? type.type) ?? (() => null);
  console.log(create);
  return (await create()) ?? null;
}
