<script lang="ts">
    import type { PageData } from './$types'

    const {data} = $props();
</script>

<h1>Welcome to SvelteKit</h1>
<p>Visit <a href="https://kit.svelte.dev">kit.svelte.dev</a> to read the documentation</p>
{ JSON.stringify(data.docs) }
