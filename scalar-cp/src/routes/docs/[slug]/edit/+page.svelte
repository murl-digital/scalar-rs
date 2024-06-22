<script lang="ts">
import type { PageData } from './$types';

    const { data } = $props();

    let formData = $state({});

    $inspect(formData)

    for (const field of data.schema.fields) {

        switch (field.field_type.type) {
            case "Integer":
                formData[field.name] = field.field_type.default;
                break;
        
            default:
                formData[field.name] = null;
                break;
        }
    }
</script>

{#each data.schema.fields as field}
    {field.title}
    {#if field.field_type.type == "SingleLine"}
        <input bind:value={formData[field.name]}>
    {:else if field.field_type.type == "Integer"}
        <input type="number" step="1" bind:value={formData[field.name]}>
    {/if}
{/each}