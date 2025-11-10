<script lang="ts">
    import { Spring } from "svelte/motion";

    let { pos }: { pos: { x: number; y: number } } = $props();

    let spring = Spring.of(() => pos);
</script>

<div
    class="absolute pointer-events-none w-4 h-4 bg-white rounded-full -translate-x-1/2 -translate-y-1/2"
    style:left={`${spring.current.x}%`}
    style:top={`${spring.current.y}%`}
></div>
