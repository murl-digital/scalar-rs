import type { EditorField } from "$ts/EditorField";
import type { Component } from "svelte";

export type FieldComponent = Component<
  { field: EditorField; data: any },
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
        component: (await import("./components/EnumDropdown.svelte")).default,
      };
    },
  ],
  [
    "Bool",
    async () => {
      return {
        component: (await import("./components/Checkbox.svelte")).default,
      };
    },
  ],
  [
    "Integer",
    async () => {
      return {
        component: (await import("./components/IntegerInput.svelte")).default,
      };
    },
  ],
  [
    "Float",
    async () => {
      return {
        component: (await import("./components/FloatInput.svelte")).default,
      };
    },
  ],
  [
    "SingleLine",
    async () => {
      return {
        component: (await import("./components/SingleLineInput.svelte"))
          .default,
      };
    },
  ],
  [
    "MultiLine",
    async () => {
      return {
        component: (await import("./components/MultiLineInput.svelte")).default,
      };
    },
  ],
  [
    "Markdown",
    async () => {
      return {
        component: (await import("./components/MarkdownInput.svelte")).default,
      };
    },
  ],
  [
    "DateTime",
    async () => {
      return {
        component: (await import("./components/DateTimeInput.svelte")).default,
      };
    },
  ],
]);

export async function getComponent(
  field: EditorField,
): Promise<ComponentMeta | null> {
  let create = components.get(field.field_type.type) ?? (() => null);
  return (await create()) ?? null;
}
