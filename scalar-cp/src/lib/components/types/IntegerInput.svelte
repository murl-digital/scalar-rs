<script lang="ts">
    import type { EditorField } from "scalar-types";
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
    <input
        class="input-base"
        id={field.name}
        type="number"
        step="1"
        bind:value={data}
    />
</Label>
