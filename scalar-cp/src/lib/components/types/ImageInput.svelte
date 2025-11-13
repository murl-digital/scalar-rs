<script lang="ts">
    import type { EditorField } from "$ts/EditorField";
    import { error } from "@sveltejs/kit";
    import { filedrop } from "filedrop-svelte";
    import { fade } from "svelte/transition";
    import { flyAndScale } from "$lib/utils";
    import { base } from "$app/paths";
    import { apiFetch, wire } from "$lib/api";
    import Field from "../Field.svelte";
    import { Dialog, Tabs } from "bits-ui";
    import Label from "../Label.svelte";

    let {
        field,
        data = $bindable(),
        ready,
    }: { field: EditorField; data: any; ready: () => void } = $props();

    let innerReady = $state(false);

    if (
        field.field_type.type !== "struct" ||
        field.field_type.component_key !== "image"
    ) {
        error(500, "ImageInput was not given an image field");
    }

    let additionalData = field.field_type.fields.find(
        (elem) => elem.name === "additional_data",
    );

    if (!additionalData) {
        error(500, "ImageInput was not given an additional_data field");
    }

    if (!data) {
        data = {
            url: null,
            additional_data: null,
        };
    }

    if (additionalData.field_type.type === "null") {
        data["additional_data"] = null;
        innerReady = true;
    }

    $effect(() => {
        if (innerReady) {
            ready();
        }
    });

    let uploadProgress = $state(0);
    let tab: "upload" | "select" = $state("upload");
    let open = $state(false);

    async function uploadFile(f: Blob) {
        const xhr = new XMLHttpRequest();
        const success = await new Promise((resolve) => {
            xhr.open("PUT", `${base}/api/images/upload`, true);
            wire(xhr);
            xhr.upload.onprogress = (event) => {
                if (event.lengthComputable) {
                    console.log("upload progress:", event.loaded / event.total);
                    uploadProgress = event.loaded / event.total;
                }
            };

            xhr.onloadend = () => {
                resolve(xhr.readyState === 4 && xhr.status === 200);
                data["url"] = xhr.responseText;
            };
            xhr.setRequestHeader("Content-Type", "application/octet-stream");
            xhr.send(f);
        });
        console.log(success);
    }
</script>

<Label {field}>
    {#if data.url}
        <!-- svelte-ignore a11y_missing_attribute -->
        <img src={data.url} />
    {/if}
    <Dialog.Root bind:open>
        <div class="flex flex-row gap-2">
            <Dialog.Trigger
                onclick={() => (tab = "upload")}
                class="input-button"
            >
                Upload
            </Dialog.Trigger>
            <Dialog.Trigger
                onclick={() => (tab = "select")}
                class="input-button"
            >
                Select Existing
            </Dialog.Trigger>
            <Dialog.Portal>
                <Dialog.Overlay
                    class="fixed inset-0 z-50 backdrop-blur-xs backdrop-brightness-50"
                />
                <Dialog.Content
                    class="fixed z-50 left-1/2 top-1/2 max-h-[85vh] max-w-[90vw] flex
                            min-w-min -translate-x-1/2 -translate-y-1/2 p-8 bg-dark text-gray border-1 rounded-xs shadow-lg shadow-black transition-all"
                >
                    <Tabs.Root bind:value={tab}>
                        <Tabs.List>
                            <Tabs.Trigger value="upload">Upload</Tabs.Trigger>
                            <Tabs.Trigger value="select">
                                Select Existing
                            </Tabs.Trigger>
                        </Tabs.List>
                        <Tabs.Content value="upload">
                            <div
                                use:filedrop={{ fileLimit: 1 }}
                                onfiledrop={(e) =>
                                    uploadFile(e.detail.files.accepted[0])}
                                class="input-base w-sm h-16"
                            ></div>
                            <progress value={uploadProgress}> </progress>
                        </Tabs.Content>
                        <Tabs.Content
                            value="select"
                            class="flex-auto overflow-scroll"
                        >
                            {#await apiFetch(fetch, `${base}/api/images/list`).then( (r) => r.json(), )}
                                ...
                            {:then obj}
                                <div class="grid grid-cols-3">
                                    {#each obj as url}
                                        <button
                                            onclick={() => {
                                                data.url = url;
                                                open = false;
                                            }}
                                            aria-label="select image"
                                        >
                                            <!-- svelte-ignore a11y_missing_attribute -->
                                            <img src={url} />
                                        </button>
                                    {/each}
                                </div>
                            {/await}
                        </Tabs.Content>
                    </Tabs.Root>
                </Dialog.Content>
            </Dialog.Portal>
        </div>
    </Dialog.Root>
    <div class="border">
        {#if additionalData.field_type.type !== "null"}
            <Field
                field={additionalData}
                bind:data={data["additional_data"]}
                ready={() => (innerReady = true)}
            ></Field>
        {/if}
    </div>
</Label>
