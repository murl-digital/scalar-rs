<script lang="ts">
    import Field from '$lib/components/Field.svelte';
import type { PageData } from './$types';

    const { data } = $props();

    let formData = $state({});

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

    $inspect(formData);
    $inspect(data);
</script>

{#each data.schema.fields as field}
    <Field bind:data={formData} field={field}></Field>
{/each}