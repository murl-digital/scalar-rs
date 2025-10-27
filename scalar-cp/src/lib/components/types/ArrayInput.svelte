<script lang="ts">
    import type { EditorField } from "$ts/EditorField";
    import { error } from "@sveltejs/kit";
    import { getComponent, type ComponentMeta } from "$lib/field-component";
    import { onMount } from "svelte";
    import { flip } from "svelte/animate";
    import { SortableItem } from "svelte-sortable-items";

    let {
        field,
        data = $bindable(),
        ready,
    }: { field: EditorField; data: any[]; ready: () => void } = $props();

    if (data == null) {
        data = [];
    }

    let internalArray = $state(
        data.map((v, i) => {
            return { id: i, v: v };
        }),
    );
    let currentHovered = $state(-1);

    $effect(() => {
        data = internalArray.map((v) => v.v);
    });

    if (field.field_type.type != "array") {
        error(500, "invalid field type");
    }

    let meta: Promise<ComponentMeta | null> = $derived(
        getComponent(field.field_type.of),
    );

    let of: EditorField = {
        name: "",
        title: "",
        required: true,
        placeholder: null,
        validator: null,
        field_type: field.field_type.of,
    };

    onMount(() => {
        ready();
    });
</script>

<ol class="flex flex-col gap-2">
    {#each internalArray as elem, i (elem.id)}
        <li animate:flip>
            <SortableItem
                propItemNumber={i}
                bind:propHoveredItemNumber={currentHovered}
                bind:propData={internalArray}
                class={[
                    "flex items-center border-base transition-all p-2",
                    i == currentHovered && "border-active",
                ]}
            >
                <div
                    class="i-ph-dots-six-vertical-bold hover:cursor-grab"
                ></div>
                {#await meta}
                    <div class="i-svg-spinners-90-ring"></div>
                {:then meta}
                    {#if meta}
                        <meta.component
                            field={of}
                            bind:data={internalArray[i].v}
                            ready={() => {}}
                        />
                    {:else}
                        <div>
                            !! WARNING !! component for {field.field_type.type} not
                            found
                        </div>
                    {/if}
                {/await}
                <button
                    class="input-button"
                    onclick={() => internalArray.splice(i, 1)}>Remove</button
                >
            </SortableItem>
        </li>
    {/each}
</ol>
<button
    class="input-button"
    onclick={() =>
        internalArray.push({
            id:
                internalArray.reduce((a, b) => Math.max(a, b.id), -Infinity) +
                1,
            v: null,
        })}>Add</button
>
