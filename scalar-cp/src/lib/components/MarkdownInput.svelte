<script lang="ts">
    import type { EditorField } from "$ts/EditorField";
    import { Carta, MarkdownEditor } from "carta-md";
    import DOMPurify from "isomorphic-dompurify";
    import "carta-md/default.css";

    let { field, data = $bindable() }: { field: EditorField; data: any } =
        $props();

    const carta = new Carta({
        sanitizer: DOMPurify.sanitize,
    });
</script>

<MarkdownEditor {carta} bind:value={data[field.name]} />

<style>
    /* Set your monospace font (Required to have the editor working correctly!) */
    /* :global(.carta-font-code) {
        font-family: "...", monospace;
        font-size: 1.1rem;
    } */

    /* Editor dark mode */
    /* Only if you are using the default theme */
    :global(.carta-theme__default) {
        --border-color: var(--border-color-dark);
        --selection-color: var(--selection-color-dark);
        --focus-outline: var(--focus-outline-dark);
        --hover-color: var(--hover-color-dark);
        --caret-color: var(--caret-color-dark);
        --text-color: var(--text-color-dark);
    }

    /* Code dark mode */
    /* Only if you didn't specify a custom code theme */
    :global(.shiki),
    :global(.shiki span) {
        color: var(--shiki-dark) !important;
    }
</style>
