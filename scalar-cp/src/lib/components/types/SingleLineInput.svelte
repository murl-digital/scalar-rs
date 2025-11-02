<script lang="ts">
    import type { EditorField } from "$ts/EditorField";
    import { onMount } from "svelte";
    import Label from "../Label.svelte";

    let {
        field,
        data = $bindable(),
        ready,
    }: { field: EditorField; data: any; ready: () => void } = $props();

    onMount(() => {
        ready();
    });
</script>

<Label {field}>
    <input class="input-base p-2" id={field.name} bind:value={data} />
</Label>
