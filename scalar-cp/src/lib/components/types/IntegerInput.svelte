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
    <input
        class="input-base"
        id={field.name}
        type="number"
        step="1"
        bind:value={data}
    />
</label>
