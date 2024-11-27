<script lang="ts">
    import type { EditorField } from "$ts/EditorField";

    let { field, data = $bindable() }: { field: EditorField; data: any } =
        $props();
</script>

<input id={field.name} type="checkbox" bind:checked={data[field.name]} />
