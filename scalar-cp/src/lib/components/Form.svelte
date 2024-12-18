<script lang="ts">
    import type { EditorField } from "$ts/EditorField";
    import Field from "./Field.svelte";

    let ready_ids = new Set();

    let {
        fields,
        formData = $bindable(),
        ready,
    }: { fields: EditorField[]; formData: any; ready: () => void } = $props();

    for (let field of fields) {
        if (formData[field.name] === undefined) {
            formData[field.name] = null;
        }
    }

    $inspect(ready_ids);

    function check() {
        console.log(ready_ids.size);
        if (ready_ids.size == fields.length) {
            ready();
        }
    }
</script>

<form class="flex flex-col gap-6">
    {#each fields as field}
        <Field
            bind:data={formData[field.name]}
            {field}
            ready={() => {
                ready_ids.add(field.name);
                console.log(ready_ids);
                check();
            }}
        ></Field>
    {/each}
</form>
