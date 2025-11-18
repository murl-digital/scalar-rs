<script lang="ts">
    import type { EditorField } from "scalar-types";
    import { onMount } from "svelte";
    import Field from "../Field.svelte";
    import { error } from "@sveltejs/kit";
    import { Checkbox } from "bits-ui";
    import Label from "../Label.svelte";

    let {
        field,
        data = $bindable(),
        ready,
    }: { field: EditorField; data: any; ready: () => void } = $props();

    if (field.field_type.type != "toggle") {
        error(500, "invalid field type");
    }

    let inner: EditorField = {
        name: "",
        title: "",
        required: true,
        placeholder: null,
        validator: null,
        field_type: field.field_type.value,
    };

    let checked = $state(data == null);

    $effect(() => {
        if (!checked) {
            data = null;
        }
    });

    onMount(() => {
        if (!checked) {
            ready();
        }
    });
</script>

<Label {field}>
    <Checkbox.Root
        bind:checked
        class="flex size-5 appearance-none items-center justify-center input-base"
    >
        {#snippet children({ checked, indeterminate })}
            {#if indeterminate}
                <div class="i-ph-minus pointer-events-none"></div>
            {:else if checked}
                <div class="i-ph-check pointer-events-none"></div>
            {/if}
        {/snippet}
    </Checkbox.Root>
</Label>
{#if checked}
    <Field field={inner} bind:data {ready}></Field>
{/if}
