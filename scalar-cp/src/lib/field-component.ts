import type { EditorField } from "$ts/EditorField";
import type { EditorType } from "$ts/EditorType";
import type { Component } from "svelte";

export type FieldComponent = Component<
  { field: EditorField; data: any; ready: () => void },
  {},
  "data"
>;

export type ComponentMeta = {
  component: FieldComponent;
};

const components: Map<string, () => Promise<ComponentMeta>> = new Map([
  [
    "Enum",
    async () => {
      return {
        component: (await import("./components/types/EnumDropdown.svelte"))
          .default,
      };
    },
  ],
  [
    "Bool",
    async () => {
      return {
        component: (await import("./components/types/BoolInput.svelte"))
          .default,
      };
    },
  ],
  [
    "Integer",
    async () => {
      return {
        component: (await import("./components/types/IntegerInput.svelte"))
          .default,
      };
    },
  ],
  [
    "Float",
    async () => {
      return {
        component: (await import("./components/types/FloatInput.svelte"))
          .default,
      };
    },
  ],
  [
    "SingleLine",
    async () => {
      return {
        component: (await import("./components/types/SingleLineInput.svelte"))
          .default,
      };
    },
  ],
  [
    "MultiLine",
    async () => {
      return {
        component: (await import("./components/types/MultiLineInput.svelte"))
          .default,
      };
    },
  ],
  [
    "Markdown",
    async () => {
      return {
        component: (await import("./components/types/MarkdownInput.svelte"))
          .default,
      };
    },
  ],
  [
    "DateTime",
    async () => {
      return {
        component: (await import("./components/types/DateTimeInput.svelte"))
          .default,
      };
    },
  ],
  [
    "Array",
    async () => {
      return {
        component: (await import("./components/types/ArrayInput.svelte"))
          .default,
      };
    },
  ],
]);

export async function getComponent(
  type: EditorType,
): Promise<ComponentMeta | null> {
  let create = components.get(type.type) ?? (() => null);
  return (await create()) ?? null;
}
