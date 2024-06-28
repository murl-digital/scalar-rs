<script lang="ts">
    import '@unocss/reset/sanitize/sanitize.css';

    import type { PageData } from './$types'

    const {data, children} = $props();
</script>

<div class="flex flex-row">
    <div>
        <ul class="flex flex-col">
            {#each data.docs as doc}
                <a class="flex items-center gap-2 flex-row ws-nowrap" href="/docs/{doc.identifier}"><div class="i-ph-files"></div>{doc.title}</a>
            {/each}
        </ul>
    </div>
    <div>
        {@render children()}
    </div>
</div>