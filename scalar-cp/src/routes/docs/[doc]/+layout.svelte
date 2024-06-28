<script lang="ts">
    import type { PageData } from './$types';
    import { page } from '$app/stores';

    const {data, children} = $props();
</script>

<div class="flex flex-row">
    <div class="b-green b-opacity-100 b-5 p-3">
        <ul class="flex flex-col">
            {#each data.docs as doc}
                {@const name = doc[Object.keys(doc).filter((key) => !key.startsWith("__sc"))[0]]}
                <a class="text-gray no-underline flex flex-row items-center gap-2" href="/docs/{$page.params.doc}/{doc.__sc_id}/edit"><div class="i-ph-file"></div>{name}</a>
            {/each}
        </ul>
    </div>
    <div>
        {@render children()}
    </div>
</div>