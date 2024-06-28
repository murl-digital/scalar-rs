<script lang="ts">
    import Field from '$lib/components/Field.svelte';
    import { invalidateAll } from '$app/navigation';
    import { page } from '$app/stores';
    import type { PageData } from './$types';

    const { data } = $props();

    let formData = $state({});
    
    async function submit(event) {
        event.preventDefault();
        await fetch(`http://localhost:3000/docs/${$page.params.doc}`, {
            method: "POST",
            headers: {
                "Content-Type": "application/json"
            },
            body: JSON.stringify(formData)
        });

        invalidateAll();
    }

    $inspect(formData);
    $inspect(data);
</script>

<form onsubmit={submit}>
    {#each data.schema.fields as field}
        <Field bind:data={formData} field={field}></Field>
    {/each}
    <button type="submit">send that shit</button>
</form>