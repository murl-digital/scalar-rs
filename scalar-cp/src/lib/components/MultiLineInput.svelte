<script lang="ts">
    import type { EditorField } from "$ts/EditorField";

    let { field, data = $bindable() }: { field: EditorField; data: any } =
        $props();
</script>

<textarea id={field.name} bind:value={data[field.name]}> </textarea>
