<script lang="ts">
    import type { EditorField } from "$ts/EditorField";
    import { createCheckbox, melt } from "@melt-ui/svelte";
    import { onMount } from "svelte";
    import Field from "../Field.svelte";

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

    const {
        elements: { root, input },
        helpers: { isChecked, isIndeterminate },
    } = createCheckbox({
        defaultChecked: data == null,
    });

    $effect(() => {
        if (!$isChecked) {
            data = null;
        }
    });

    onMount(() => {
        if (!$isChecked) {
            ready();
        }
    });
</script>

<label class="flex flex-col">
    {field.title}
    <button
        use:melt={$root}
        class="flex size-5 appearance-none items-center justify-center input-base"
    >
        {#if $isIndeterminate}
            <div class="i-ph-minus pointer-events-none"></div>
        {:else if $isChecked}
            <div class="i-ph-check pointer-events-none"></div>
        {/if}
    </button>

    {#if $isChecked}
        <Field field={inner} bind:data {ready}></Field>
    {/if}
</label>
