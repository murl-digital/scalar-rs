<script lang="ts">
import Field from "./Field.svelte";

    let {field, data = $bindable() } = $props();

    let struct_fields = $derived(field.field_type.variants.filter((i) => i.variant_name === data[field.name]?.type)[0]?.fields ?? []);

    // this ensures that the object always has accurate data
    $effect(() => {
        Object.keys(data[field.name]).filter((key) => key !== 'type').forEach((key) => {
            if (struct_fields && !struct_fields.map((field) => field.name).includes(key)) {
                delete data[field.name][key];
            }
        });

        if (struct_fields) {
            struct_fields.forEach(i_field => {
                if (!data[field.name][i_field.name]) {
                    data[field.name][i_field.name] = null;
                }
            });
        }
    });
</script>

<select bind:value={data[field.name].type}>
    {#each field.field_type.variants as variant}
        <option  value={variant.variant_name}>{variant.variant_name}</option>
    {/each}
</select>

{#if struct_fields}
    {#each struct_fields as inner_field} 
        <Field field={inner_field} bind:data={data[field.name]}></Field>
    {/each}
{/if}