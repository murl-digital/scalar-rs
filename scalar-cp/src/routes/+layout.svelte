<script lang="ts">
    import "@unocss/reset/tailwind.css";
    import "$lib/css/base.css";

    import type { PageData } from "./$types";

    const { data, children }: { data: PageData; children: any } = $props();
</script>

<div
    class="grid grid-cols-[1fr] grid-rows-[4rem_1fr_1fr] h-screen overflow-hidden bg-dark"
>
    <div class="b-b-solid b-b-1 row-span-1 flex">
        <img class="ml-auto" src={data.avatarUrl} alt="user profile" />
    </div>
    <div class="row-start-2 row-span-2 col-span-1 overflow-scroll flex">
        <div class="p-4 b-r-solid b-r-1">
            <h1 class="text-sm">COLLECTIONS</h1>
            <ul class="flex flex-col gap-2 pl-2">
                {#each data.docs as doc}
                    <a
                        class="flex items-center gap-2 flex-row ws-nowrap text-gray b-solid b-2 p-1 px-2"
                        href="/docs/{doc.identifier}"
                        ><div class="i-ph-folder-open"></div>
                        {doc.title}</a
                    >
                {/each}
            </ul>
        </div>
        <div class="w-full flex-[1_0_0]">
            {@render children()}
        </div>
    </div>
</div>
