<script lang="ts">
    import type { EditorField } from "$ts/EditorField";
    import { getComponent, type ComponentMeta } from "$lib/field-component";
    import { onMount } from "svelte";

    let {
        field,
        data = $bindable(),
        ready,
    }: {
        field: EditorField;
        data: any;
        ready: () => void;
    } = $props();

    let meta: Promise<ComponentMeta | null> = $derived(
        getComponent(field.field_type),
    );

    // if (data == null) {
    //     console.log(field.name);
    //     switch (field.field_type.type) {
    //         case "Enum":
    //             if (field.field_type.default) {
    //                 data = field.field_type.default;
    //             } else {
    //                 data = {
    //                     type: "",
    //                 };
    //             }
    //             break;

    //         default:
    //             data = field.field_type.default;
    //             break;
    //     }
    // }
    //
</script>

{#await meta}
    <div class="i-svg-spinners-90-ring"></div>
{:then meta}
    {#if meta}
        <meta.component {field} bind:data ready={() => ready()} />
    {:else}
        <div>
            !! WARNING !! component for {field.field_type.component_key ??
                field.field_type.type} not found
        </div>
    {/if}
{:catch ex}
    <span>{JSON.stringify(ex)}</span>
{/await}
