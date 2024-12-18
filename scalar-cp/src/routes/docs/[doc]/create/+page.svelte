<script lang="ts">
    import { apiFetch } from "$lib/api";
    import { goto, invalidateAll } from "$app/navigation";
    import { page } from "$app/stores";
    import type { PageData } from "./$types";
    import { tick, untrack } from "svelte";
    import Form from "$lib/components/Form.svelte";
    import { base } from "$app/paths";
    import { nanoid } from "nanoid";

    const { data } = $props();

    let formData = $state({});
    let ready = $state(false);
    let timeout: Timer | undefined = $state();

    $effect(() => {
        clearTimeout(untrack(() => timeout));
        timeout = undefined;
        console.log("trigger");

        let init = {
            method: "PUT",
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify(formData),
        };

        if (untrack(() => ready)) {
            console.log("timeout set");
            timeout = setTimeout(() => {
                create(init).then((id) =>
                    goto(`./${id}/edit`, { invalidateAll: true }),
                );
            }, 500);
        }
    });

    async function create(init: RequestInit) {
        let id = nanoid();
        await apiFetch(
            fetch,
            `${base}/api/docs/${$page.params.doc}/drafts/${id}`,
            init,
        );

        return id;
    }

    $inspect(formData);
</script>

<div class="w-full h-full flex">
    <div class="w-full overflow-scroll">
        <div class="w-1/3 mx-auto py-16">
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
</div>
