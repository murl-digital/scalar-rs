<script lang="ts">
    import type { EditorField } from "$ts/EditorField";
    import { error } from "@sveltejs/kit";
    import { getComponent, type ComponentMeta } from "$lib/field-component";
    import { onMount } from "svelte";

    let {
        field,
        data = $bindable(),
        ready,
    }: { field: EditorField; data: any; ready: () => void } = $props();

    if (data == null) {
        data = [];
    }

    if (field.field_type.type != "array") {
        error(500, "invalid field type");
    }

    let meta: Promise<ComponentMeta | null> = $derived(
        getComponent(field.field_type.of),
    );

    let of: EditorField = {
        name: "",
        title: "",
        required: true,
        placeholder: null,
        validator: null,
        field_type: field.field_type.of,
    };

    onMount(() => {
        ready();
    });
</script>

{#each data as elem, i}
    {#await meta}
        <div class="i-svg-spinners-90-ring"></div>
    {:then meta}
        {#if meta}
            <meta.component field={of} bind:data={data[i]} ready={() => {}} />
        {:else}
            <div>
                !! WARNING !! component for {field.field_type.type} not found
            </div>
        {/if}
    {/await}
{/each}
<button class="input-button" onclick={() => data.push(null)}>Add</button>
