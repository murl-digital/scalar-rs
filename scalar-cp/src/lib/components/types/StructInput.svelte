<script lang="ts">
    import type { EditorField } from "$ts/EditorField";
    import { error } from "@sveltejs/kit";
    import Field from "../Field.svelte";
    import { SvelteSet } from "svelte/reactivity";

    let {
        field,
        data = $bindable(),
        ready,
    }: { field: EditorField; data: any; ready: () => void } = $props();

    let ready_ids = $state(new SvelteSet());

    if (field.field_type.type !== "struct") {
        error(500, "StructInput was not given an image field");
    }

    if (!data) {
        data = {};
        for (let iField of field.field_type.fields) {
            data[iField.name];
        }
    }

    $effect(() => {
        if (
            field.field_type.type === "struct" &&
            ready_ids.size === field.field_type.fields.length
        ) {
            ready();
        }
    });
</script>

{#if field.field_type.type === "struct"}
    {#each field.field_type.fields as iField}
        <Field
            field={iField}
            bind:data={data[iField.name]}
            ready={() => ready_ids.add(iField.name)}
        ></Field>
    {/each}
{/if}
