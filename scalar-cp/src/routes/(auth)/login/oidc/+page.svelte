<script lang="ts">
    import type { PageProps } from "./$types";
    import { state as appState } from "$lib/state.svelte";
    import { goto } from "$app/navigation";
    import { onMount } from "svelte";

    let { data }: PageProps = $props();

    $inspect(data);

    onMount(() => {
        if (data.token) {
            sessionStorage.setItem("token", data.token);
            appState.authenticated = true;
            goto(`/`, { invalidateAll: true });
        }
    });
</script>
