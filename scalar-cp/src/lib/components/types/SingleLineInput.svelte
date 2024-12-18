<script lang="ts">
    import type { EditorField } from "$ts/EditorField";
    import { onMount } from "svelte";

    let {
        field,
        data = $bindable(),
        ready,
    }: { field: EditorField; data: any; ready: () => void } = $props();

    onMount(() => {
        ready();
    });
</script>

<label class="flex flex-col">
    {field.title}
    <input class="input-base p-2" id={field.name} bind:value={data} />
</label>
