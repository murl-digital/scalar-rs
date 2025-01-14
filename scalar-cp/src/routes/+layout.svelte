<script lang="ts">
    import "@unocss/reset/tailwind.css";
    import "$lib/css/base.css";

    let { children } = $props();
</script>

{@render children()}
