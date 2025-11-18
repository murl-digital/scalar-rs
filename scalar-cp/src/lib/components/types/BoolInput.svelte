<script lang="ts">
    import type { EditorField } from "scalar-types";
    import { Checkbox } from "bits-ui";
    import { onMount } from "svelte";
    import Label from "../Label.svelte";

    let {
        field,
        data = $bindable(),
        ready,
    }: { field: EditorField; data: any; ready: () => void } = $props();

    let indeterminate = $state(data == null);
    let value = $state(data);

    // svelte stop throwing a temper tantrum challenge (impossible)
    const getValue = () => value || false;
    const setValue = (newValue: boolean) => {
        value = newValue;
        data = newValue;
    };

    onMount(() => {
        ready();
    });
</script>

<Label {field}>
    <Checkbox.Root
        bind:indeterminate
        bind:checked={getValue, setValue}
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
