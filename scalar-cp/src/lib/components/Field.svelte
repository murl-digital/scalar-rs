<script lang="ts">
    import type { EditorField } from "$ts/EditorField";
    import { getComponent, type ComponentMeta } from "$lib/field-component";
    import { onMount } from "svelte";

    let {
        field,
        error,
        data = $bindable(),
        ready,
    }: {
        field: EditorField;
        error: string | undefined;
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
    <div class={[error && "border-2 border-red", "p-2"]}>
        {#if error}
            <span class="text-red">{error}</span>
        {/if}
        {#if meta}
            <meta.component {field} bind:data ready={() => ready()} />
        {:else}
            <div>
                !! WARNING !! component for {field.field_type.component_key ??
                    field.field_type.type} not found
            </div>
        {/if}
    </div>
{:catch ex}
    <span>{JSON.stringify(ex)}</span>
{/await}
