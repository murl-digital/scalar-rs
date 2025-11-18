<script lang="ts">
    import type { EditorField } from "scalar-types";
    import { getComponent, type ComponentMeta } from "$lib/field-component";
    import { onMount } from "svelte";
    import type { Errors } from "$lib/types";

    let {
        field,
        errors,
        data = $bindable(),
        ready,
    }: {
        field: EditorField;
        errors?: string | Errors;
        data: any;
        ready: () => void;
    } = $props();

    let meta: Promise<ComponentMeta | null> = $derived(
        getComponent(field.field_type),
    );

    let singleError: string | undefined = $derived(
        typeof errors === "string" ? errors : undefined,
    );

    $inspect(singleError);

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
    <div class={[singleError && "border-2 border-red", "p-2"]}>
        {#if singleError}
            <span class="text-red">{singleError}</span>
        {/if}
        {#if meta}
            <meta.component
                {field}
                errors={typeof errors !== "string" ? errors : undefined}
                bind:data
                ready={() => ready()}
            />
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
