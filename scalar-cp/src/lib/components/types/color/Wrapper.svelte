<script lang="ts">
    import type { Snippet } from "svelte";

    let {
        wrapper = $bindable(),
        isOpen,
        isDialog,
        children,
    }: {
        wrapper: HTMLElement | undefined;
        isOpen: boolean;
        isDialog: boolean;
        children: Snippet;
    } = $props();
</script>

<div
    class="border relative w-max p-2 inline-block"
    style:--picker-radius="0px"
    bind:this={wrapper}
>
    {@render children()}
</div>
