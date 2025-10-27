<script lang="ts">
    import { createDropdownMenu, melt } from "@melt-ui/svelte";
    import type { PageData } from "./$types";

    const { data, children }: { data: PageData; children: any } = $props();

    const {
        elements: { menu, item, trigger, arrow },
    } = createDropdownMenu();
</script>

<div
    class="grid grid-cols-[1fr] grid-rows-[4rem_1fr_1fr] h-screen overflow-hidden bg-dark"
>
    <div class="b-b-solid b-b-1 row-span-1 flex flex-row-reverse">
        <button use:melt={$trigger} class="aspect-1">
            <img src={data.avatarUrl} alt="user profile" />
        </button>
    </div>
    <div use:melt={$menu}>
        <div use:melt={$arrow}></div>
        <div use:melt={$item}>
            <a href="/profile">Profile</a>
        </div>
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
