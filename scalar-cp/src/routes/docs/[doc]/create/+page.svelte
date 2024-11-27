<script lang="ts">
    import Field from "$lib/components/Field.svelte";
    import { goto, invalidateAll } from "$app/navigation";
    import { page } from "$app/stores";
    import type { PageData } from "./$types";
    import { untrack } from "svelte";
    import Form from "$lib/components/Form.svelte";

    const { data } = $props();

    let formData = $state({});
    let justMounted = $state(true);
    let timeout: Timer | undefined = $state();

    $effect(() => {
        clearTimeout(untrack(() => timeout));

        let init = {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify(formData),
        };

        if (untrack(() => justMounted)) {
            justMounted = false;
        } else {
            timeout = setTimeout(() => {
                fetch("./create", init)
                    .then((r) => r.text())
                    .then((id) =>
                        goto(`./${id}/edit`, { invalidateAll: true }),
                    );
            }, 500);
        }
    });

    $inspect(formData);
    $inspect(data);
</script>

<Form fields={data.schema.fields} bind:formData></Form>
