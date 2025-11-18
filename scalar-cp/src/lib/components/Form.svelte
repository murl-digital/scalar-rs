<script lang="ts">
    import type { EditorField } from "scalar-types";
    import { SvelteSet } from "svelte/reactivity";
    import Field from "./Field.svelte";
    import type { Errors } from "$lib/types";

    let ready_ids = $state(new SvelteSet());

    let {
        fields,
        errors,
        formData = $bindable(),
        ready,
    }: {
        fields: EditorField[];
        errors: Errors;
        formData: any;
        ready: () => void;
    } = $props();

    for (let field of fields) {
        if (formData[field.name] === undefined) {
            formData[field.name] = null;
        }
    }

    $inspect(ready_ids);

    $effect(() => {
        if (ready_ids.size == fields.length) {
            ready();
        }
    });
</script>

<form class="flex flex-col gap-6">
    {#each fields as field}
        <Field
            bind:data={formData[field.name]}
            errors={errors.find((f) => f.field === field.name)?.error}
            {field}
            ready={() => {
                ready_ids.add(field.name);
            }}
        ></Field>
    {/each}
</form>
