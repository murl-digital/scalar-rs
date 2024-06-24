<script lang="ts">
    let {field, data = $bindable()} = $props();

    $inspect(data);
</script>

{#if data} 
    {field.title}
    {#if field.field_type.type == "SingleLine"}
        <input bind:value={data[field.name]}>
    {:else if field.field_type.type == "Integer"}
        <input type="number" step="1" bind:value={data[field.name]}>
    {:else if field.field_type.type == "Enum"}
        <select bind:value={data[field.name]}>
            {#each field.field_type.variants as variant}
                <option value={{type: variant.variant_name}}>{variant.variant_name}</option>
            {/each}
        </select>

        {@const struct_fields = field.field_type.variants.filter((i) => i.variant_name === data[field.name]?.type)[0]?.fields}
        {#if struct_fields}
            {#each struct_fields as inner_field} 
                {#key data[field.name]}
                    <svelte:self field={inner_field} bind:data={data[field.name]}></svelte:self>
                {/key}
            {/each}
        {/if}
    {/if}
{/if}