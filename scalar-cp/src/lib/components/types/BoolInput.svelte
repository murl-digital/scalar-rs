<script lang="ts">
    import type { EditorField } from "$ts/EditorField";
    import { createCheckbox, melt } from "@melt-ui/svelte";
    import { onMount } from "svelte";

    let {
        field,
        data = $bindable(),
        ready,
    }: { field: EditorField; data: any; ready: () => void } = $props();

    const {
        elements: { root, input },
        helpers: { isChecked, isIndeterminate },
    } = createCheckbox({
        defaultChecked: data == null ? "indeterminate" : data,
    });

    $effect(() => {
        data = $isIndeterminate ? null : $isChecked;
    });

    onMount(() => {
        ready();
    });
</script>

<label class="flex flex-col">
    {field.title}
    <button
        use:melt={$root}
        class="flex size-5 appearance-none items-center justify-center bg-neutral-700 outline outline-1 outline-gray text-white rounded-sm ring ring-transparent hover:ring-purple focus:ring-purple focus-visible:ring-purple ring-offset-2 ring-offset-dark ring-2"
    >
        {#if $isIndeterminate}
            <div class="i-ph-minus pointer-events-none"></div>
        {:else if $isChecked}
            <div class="i-ph-check pointer-events-none"></div>
        {/if}
    </button>
</label>
