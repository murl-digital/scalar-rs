<script lang="ts">
    import Field from "$lib/components/Field.svelte";
    import { untrack } from "svelte";
    import type { PageData } from "./$types";
    import { wait } from "$lib/utils";
    import { slide } from "svelte/transition";
    import Form from "$lib/components/Form.svelte";

    const { data }: { data: PageData } = $props();

    console.log(data);

    let formData = $state(data.doc);

    let updatingPromise = $state();
    let justMounted = $state(true);

    $effect.pre(() => {
        justMounted = true;
        formData = data.doc;
    });

    $effect(() => {
        let init = {
            method: "PATCH",
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify(formData),
        };

        if (untrack(() => justMounted)) {
            justMounted = false;
        } else {
            updatingPromise = fetch("./edit", init).then((value) =>
                wait(1500, value),
            );
        }
    });

    $inspect(formData);
    $inspect(justMounted);
</script>

<div class="flex flex-col w-full h-full relative">
    <div class="w-full overflow-scroll">
        <div class="w-1/3 mx-auto">
            <Form fields={data.schema.fields} bind:formData></Form>
        </div>
    </div>

    <div class="bg-red w-full h-16 mt-auto">
        {#await updatingPromise}
            <div
                transition:slide
                class="w-lg h-lg i-svg-spinners-blocks-wave"
            ></div>
        {/await}
    </div>
</div>
