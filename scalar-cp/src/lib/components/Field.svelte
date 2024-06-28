<script lang="ts">
import EnumDropdown from "./EnumDropdown.svelte";

    let {field, data = $bindable()} = $props();

    if (!data[field.name]) {
        switch (field.field_type.type) {
            case "Integer":
                data[field.name] = field.field_type.default;
                break;

            case "Enum":
                if (field.field_type.default) {
                    data[field.name] = field.field_type.default;
                } else {
                    data[field.name] = {
                        type: ""
                    };
                }
                break;
        
            default:
                data[field.name] = null;
                break;
        }
    }
</script>

{#if data} 
    {field.title}
    {#if field.field_type.type == "SingleLine"}
        <input bind:value={data[field.name]}>
    {:else if field.field_type.type == "Integer"}
        <input type="number" step="1" bind:value={data[field.name]}>
    {:else if field.field_type.type == "Enum"}
        <EnumDropdown {field} bind:data={data}></EnumDropdown>
    {/if}
{/if}