<script lang="ts">
    import { apiFetch } from "$lib/api";
    import { untrack } from "svelte";
    import type { PageData } from "./$types";
    import { wait } from "$lib/utils";
    import { slide } from "svelte/transition";
    import Form from "$lib/components/Form.svelte";
    import { base } from "$app/paths";
    import { page } from "$app/state";

    const { data }: { data: PageData } = $props();

    console.log(data);

    let formData = $state(data.doc);

    let updatingPromise = $state();
    let ready = $state(false);

    $effect.pre(() => {
        ready = false;
        formData = data.doc;
    });

    $effect(() => {
        let init = {
            method: "PUT",
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify(formData),
        };

        if (untrack(() => ready)) {
            updatingPromise = apiFetch(
                fetch,
                `${base}/api/docs/${page.params.doc}/drafts/${page.params.doc_id}`,
                init,
            ).then((value) => wait(1500, value));
        }
    });

    $inspect(formData);
    $inspect(ready);

    let colorData = $state();
    $inspect(colorData);
</script>

<div class="flex flex-col flex-initial w-full h-full">
    <div class="w-full flex-auto overflow-scroll py-8">
        <div class="mx-auto w-1/3">
            <Form
                fields={data.schema.fields}
                bind:formData
                ready={() => {
                    ready = true;
                    console.log("ready!");
                }}
            ></Form>
        </div>
    </div>

    <div class="b-t-solid b-t-2 w-full h-32">
        {#await updatingPromise}
            <div
                transition:slide
                class="size-8 i-svg-spinners-blocks-wave"
            ></div>
        {/await}
    </div>
</div>
