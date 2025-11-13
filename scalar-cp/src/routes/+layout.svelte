<script lang="ts">
    import "@unocss/reset/tailwind-v4.css";
    import "$lib/css/base.css";
    import "$lib/css/splash.css";

    let { children } = $props();

    $effect(() => {
        let elem = document.getElementById("splash");

        if (elem) {
            elem.setAttribute("data_loaded", "true");
        }
    });
</script>

{@render children()}
