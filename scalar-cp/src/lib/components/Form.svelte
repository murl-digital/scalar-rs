<script lang="ts">
    import type { EditorField } from "$ts/EditorField";
    import Field from "./Field.svelte";

    let {
        fields,
        formData = $bindable(),
    }: { fields: EditorField[]; formData: any } = $props();
</script>

<form class="flex flex-col">
    {#each fields as field}
        <Field bind:data={formData} {field}></Field>
    {/each}
</form>
