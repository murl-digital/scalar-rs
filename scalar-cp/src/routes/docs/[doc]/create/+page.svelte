<script lang="ts">
    import { apiFetch } from "$lib/api";
    import { goto, invalidateAll } from "$app/navigation";
    import { page } from "$app/stores";
    import type { PageData } from "./$types";
    import { untrack } from "svelte";
    import Form from "$lib/components/Form.svelte";
    import { base } from "$app/paths";
    import { nanoid } from "nanoid";

    const { data } = $props();

    let formData = $state({});
    let justMounted = $state(true);
    let timeout: Timer | undefined = $state();

    $effect(() => {
        clearTimeout(untrack(() => timeout));

        let init = {
            method: "PUT",
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify(formData),
        };

        if (untrack(() => justMounted)) {
            justMounted = false;
        } else {
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
    $inspect(data);
</script>

<Form fields={data.schema.fields} bind:formData></Form>
