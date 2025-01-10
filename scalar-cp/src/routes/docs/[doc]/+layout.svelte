<script lang="ts">
    import type { PageData } from "./$types";
    import { page } from "$app/state";

    const { data, children } = $props();
</script>

<div class="flex flex-row h-full w-full">
    <div class="b-r-solid b-r-1 p-3 pr-6 h-full">
        <ul class="flex flex-col pl-0 my-0">
            {#each data.docs as doc, index}
                {@const name = doc.content[Object.keys(doc.content)[0]]}
                <a
                    class="text-gray no-underline flex flex-row items-center gap-2"
                    href="/docs/{page.params.doc}/{doc.__sc_id}/edit"
                    ><div class="i-ph-file-text"></div>
                    {name}
                    {index}</a
                >
            {/each}
            <a
                class="text-gray no-underline flex flex-row items-center gap-2"
                href="/docs/{page.params.doc}/create"
            >
                <div class="i-ph-plus"></div>
                Create New
            </a>
        </ul>
    </div>
    <div class="h-full w-full">
        {@render children()}
    </div>
</div>
