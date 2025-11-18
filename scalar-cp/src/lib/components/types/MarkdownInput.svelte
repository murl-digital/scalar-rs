<script lang="ts">
    import type { EditorField } from "scalar-types";
    import { Carta, MarkdownEditor } from "carta-md";
    import DOMPurify from "isomorphic-dompurify";
    import "$lib/css/carta-theme-scalar.css";
    import { onMount } from "svelte";
    import Label from "../Label.svelte";

    let {
        field,
        data = $bindable(),
        ready,
    }: { field: EditorField; data: any; ready: () => void } = $props();

    let internalText = $state(data ?? "");

    $effect(() => {
        if (internalText) {
            data = internalText;
        } else if (data) {
            data = null;
        }
    });

    const carta = new Carta({
        sanitizer: DOMPurify.sanitize,
    });

    onMount(() => {
        ready();
    });
</script>

<Label {field}>
    <MarkdownEditor theme="scalar" {carta} bind:value={internalText} />
</Label>

<style>
    /* Set your monospace font (Required to have the editor working correctly!) */
    :global(.carta-font-code) {
        font-family: "...", monospace;
        font-size: 1.1rem;
    }

    /* :global(.carta-toolbar-left) {
        @apply text-gray!;
    } */

    /* Editor dark mode */
    /* Only if you are using the default theme */
    /* :global(.carta-theme__default) {
        --border-color: var(--border-color-dark);
        --selection-color: var(--selection-color-dark);
        --focus-outline: var(--focus-outline-dark);
        --hover-color: var(--hover-color-dark);
        --caret-color: var(--caret-color-dark);
        --text-color: var(--text-color-dark);
    } */

    /* Code dark mode */
    /* Only if you didn't specify a custom code theme */
    :global(.shiki),
    :global(.shiki span) {
        color: var(--shiki-dark) !important;
    }

    /* :global(.carta-renderer) {
        @apply prose;
    } */
</style>
