<script lang="ts">
    import type { EditorField } from "$ts/EditorField";
    import { Label } from "bits-ui";
    import type { Snippet } from "svelte";

    let { field, children }: { field?: EditorField; children: Snippet } =
        $props();
</script>

<Label.Root class="flex flex-col">
    <span>{field?.title}</span>
    {@render children()}
</Label.Root>
