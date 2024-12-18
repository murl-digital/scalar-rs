<script lang="ts">
    import { apiFetch } from "$lib/api";
    import { untrack } from "svelte";
    import type { PageData } from "./$types";
    import { wait } from "$lib/utils";
    import { slide } from "svelte/transition";
    import Form from "$lib/components/Form.svelte";
    import { base } from "$app/paths";
    import { page } from "$app/stores";

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
                `${base}/api/docs/${$page.params.doc}/drafts/${$page.params.doc_id}`,
                init,
            ).then((value) => wait(1500, value));
        }
    });

    $inspect(formData);
    $inspect(ready);
</script>

<div class="flex flex-col w-full h-full relative">
    <div class="w-full overflow-scroll">
        <div class="w-1/3 mx-auto">
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

    <div class="bg-red w-full h-16 mt-auto">
        {#await updatingPromise}
            <div
                transition:slide
                class="w-lg h-lg i-svg-spinners-blocks-wave"
            ></div>
        {/await}
    </div>
</div>
