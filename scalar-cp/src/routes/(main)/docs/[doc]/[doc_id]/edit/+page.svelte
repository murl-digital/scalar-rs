<script lang="ts">
    import { apiFetch } from "$lib/api";
    import { untrack } from "svelte";
    import type { PageData } from "./$types";
    import { wait } from "$lib/utils";
    import { fly, slide } from "svelte/transition";
    import Form from "$lib/components/Form.svelte";
    import { base } from "$app/paths";
    import { page } from "$app/state";
    import DateTimeInput from "$lib/components/types/DateTimeInput.svelte";
    import type { Errors } from "$lib/types";
    import { Popover } from "bits-ui";

    const { data }: { data: PageData } = $props();

    let formData = $state(data.doc);

    let updatingPromise = $state();
    let ready = $state(false);
    let valid = $state(false);
    let validationErrors: Errors = $state([]);
    let timeout: number | undefined = $state();

    $effect.pre(() => {
        ready = false;
        formData = data.doc;
    });

    let publishAt = $state();

    $effect(() => {
        clearTimeout(untrack(() => timeout));
        timeout = undefined;

        let init = {
            method: "PUT",
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify(formData),
        };

        if (untrack(() => ready)) {
            timeout = setTimeout(() => {
                updatingPromise = apiFetch(
                    fetch,
                    `${base}/api/docs/${page.params.doc}/drafts/${page.params.doc_id}`,
                    init,
                ).then((value) => wait(1500, value));
                apiFetch(
                    fetch,
                    `${base}/api/docs/${page.params.doc}/validate`,
                    {
                        method: "POST",
                        headers: {
                            "Content-Type": "application/json",
                        },
                        body: JSON.stringify(formData),
                    },
                ).then((response) => {
                    validationErrors = [];
                    if (response.ok) {
                        valid = true;
                    } else {
                        response.json().then((json) => {
                            valid = false;
                            validationErrors = json;
                        });
                    }
                });
            }, 500);
        }
    });

    $inspect(formData);
    $inspect(ready);
</script>

<div class="flex flex-col flex-initial w-full h-full">
    <div class="w-full flex-auto overflow-scroll py-8">
        <div class="mx-auto w-1/3">
            <Form
                fields={data.schema.fields}
                errors={validationErrors}
                bind:formData
                ready={() => {
                    ready = true;
                    console.log("ready!");
                }}
            />
        </div>
    </div>

    <div class="b-t-solid b-t-2 w-full h-32 flex p-2">
        {#await updatingPromise}
            <div
                transition:slide
                class="size-8 i-svg-spinners-blocks-wave"
            ></div>
        {/await}
        <span>
            {#if valid}
                valid!
            {:else}
                invalid!
            {/if}
        </span>
        <span class="input-border rounded-sm flex w-fit gap-1">
            <button
                class="px-2 py-1 bg-neutral-700 hover:bg-neutral-600 transition-all"
            >
                Publish
            </button>
            <Popover.Root>
                <Popover.Trigger
                    aria-label="More publish options"
                    class="px-1 py-1 bg-neutral-700 hover:bg-neutral-600 transition-all"
                >
                    <div class="i-ph-caret-up"></div>
                </Popover.Trigger>
                <Popover.Portal>
                    <Popover.Content class="border">
                        <Popover.Arrow />
                        <div>
                            <DateTimeInput
                                field={{
                                    name: "publish-at",
                                    title: "Publish At",
                                    validator: null,
                                    required: false,
                                    placeholder: null,
                                    field_type: {
                                        type: "date-time",
                                        component_key: null,
                                        default: null,
                                    },
                                }}
                                bind:data={publishAt}
                                ready={() => {}}
                            ></DateTimeInput>
                        </div>
                        <Popover.Close>Close</Popover.Close>
                    </Popover.Content>
                </Popover.Portal>
            </Popover.Root>
        </span>
    </div>
</div>
