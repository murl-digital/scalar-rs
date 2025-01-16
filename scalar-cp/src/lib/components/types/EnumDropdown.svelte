<script lang="ts">
    import type { EditorField } from "$ts/EditorField";
    import { error } from "@sveltejs/kit";
    import Field from "$lib/components/Field.svelte";
    import { onMount } from "svelte";

    let {
        field,
        data = $bindable(),
        ready,
    }: { field: EditorField; data: any; ready: () => void } = $props();

    if (field.field_type.type !== "enum") {
        error(500, "EnumDropdown was given a field that was not an enum");
    }

    if (field.field_type.default) {
        data = field.field_type.default;
    } else {
        data = {
            type: "",
        };
    }

    let struct_fields = $derived(
        field.field_type.variants.filter(
            (i) => i.variant_name === data?.type,
        )[0]?.fields ?? [],
    );

    // this ensures that the object always has accurate data
    $effect(() => {
        Object.keys(data)
            .filter((key) => key !== "type")
            .forEach((key) => {
                if (
                    struct_fields &&
                    !struct_fields.map((field) => field.name).includes(key)
                ) {
                    delete data[key];
                }
            });

        if (struct_fields) {
            struct_fields.forEach((i_field) => {
                if (!data[i_field.name]) {
                    data[i_field.name] = null;
                }
            });
        }
    });

    onMount(() => {
        ready();
    });
</script>

<label class="flex flex-col">
    {field.title}
    {#if field.field_type.type === "enum"}
        <select bind:value={data.type} class="input-base">
            {#each field.field_type.variants as variant}
                <option value={variant.variant_name}
                    >{variant.variant_name}</option
                >
            {/each}
        </select>
    {/if}

    {#if struct_fields}
        {#each struct_fields as inner_field}
            <Field
                field={inner_field}
                bind:data={data[inner_field.name]}
                ready={() => {}}
            ></Field>
        {/each}
    {/if}
</label>
