<script lang="ts">
    import type { EditorField } from "scalar-types";
    import { error } from "@sveltejs/kit";
    import Field from "../Field.svelte";
    import { SvelteSet } from "svelte/reactivity";
    import Label from "../Label.svelte";

    let {
        field,
        data = $bindable(),
        ready,
        errors,
    }: {
        field: EditorField;
        data: any;
        ready: () => void;
        errors?: [{ field: string; error: string }];
    } = $props();

    let ready_ids = new SvelteSet();

    $inspect(errors);

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

<Label {field}>
    {#if field.field_type.type === "struct"}
        <div class="border p-2">
            {#each field.field_type.fields as iField}
                <Field
                    field={iField}
                    bind:data={data[iField.name]}
                    ready={() => ready_ids.add(iField.name)}
                    errors={errors?.find((f) => f.field == iField.name)?.error}
                ></Field>
            {/each}
        </div>
    {/if}
</Label>
