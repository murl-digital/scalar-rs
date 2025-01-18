<script lang="ts">
    import "@unocss/reset/tailwind.css";
    import "$lib/css/base.css";
    import { hideSplashScreen } from "vite-plugin-splash-screen/runtime";

    let { children } = $props();

    $effect(() => {
        hideSplashScreen();
    });
</script>

{@render children()}
