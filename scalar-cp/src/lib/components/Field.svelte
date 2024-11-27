<script lang="ts">
    import type { Component } from "svelte";
    import EnumDropdown from "./EnumDropdown.svelte";
    import Checkbox from "./Checkbox.svelte";
    import type { EditorField } from "$ts/EditorField";

    import { createLabel, melt } from "@melt-ui/svelte";
    import { getComponent, type ComponentMeta } from "$lib/field-component";

    const {
        elements: { root },
    } = createLabel();

    let { field, data = $bindable() }: { field: EditorField; data: any } =
        $props();

    let meta: Promise<ComponentMeta | null> = $derived(getComponent(field));

    if (data[field.name] == null) {
        console.log(field.name);
        switch (field.field_type.type) {
            case "Enum":
                if (field.field_type.default) {
                    data[field.name] = field.field_type.default;
                } else {
                    data[field.name] = {
                        type: "",
                    };
                }
                break;

            default:
                data[field.name] = field.field_type.default;
                break;
        }
    }
</script>

{#if data}
    <!-- <label use:melt={$root} for={field.name}>{field.title}</label>
    {#if field.field_type.type == "SingleLine"}
        <input id={field.name} bind:value={data[field.name]} />
    {:else if field.field_type.type == "MultiLine"}
        <textarea id={field.name} bind:value={data[field.name]}> </textarea>
    {:else if field.field_type.type == "Markdown"}
        <span>!!TODO!!</span>
        <textarea id={field.name} bind:value={data[field.name]}> </textarea>
    {:else if field.field_type.type == "Integer"}
        <input
            id={field.name}
            type="number"
            step="1"
            bind:value={data[field.name]}
        />
    {:else if field.field_type.type == "Float"}
        <input id={field.name} type="number" bind:value={data[field.name]} />
    {:else if field.field_type.type == "Array"}
        <span>!!TODO!!</span>
    {:else}
        <test.component {field} bind:data />
    {/if} -->
    {#await meta}
        <p>loading component...</p>
    {:then meta}
        {#if meta}
            <meta.component {field} bind:data />
        {:else}
            <div>
                !! WARNING !! component for {field.field_type.type} not found
            </div>
        {/if}
    {/await}
{/if}
