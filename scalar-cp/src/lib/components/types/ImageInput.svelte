<script lang="ts">
    import type { EditorField } from "$ts/EditorField";
    import { error } from "@sveltejs/kit";
    import { createDialog, createTabs, melt } from "@melt-ui/svelte";
    import { filedrop } from "filedrop-svelte";
    import { fade } from "svelte/transition";
    import { flyAndScale } from "$lib/utils";
    import { base } from "$app/paths";
    import { apiFetch, wire } from "$lib/api";
    import { addHighlight } from "@melt-ui/svelte/internal/helpers";
    import Field from "../Field.svelte";

    const {
        elements: {
            trigger,
            portalled,
            overlay,
            content,
            title,
            description,
            close,
        },
        states: { open },
    } = createDialog();

    const {
        elements: { root, list, content: tab_content, trigger: tab_trigger },
        states: { value },
    } = createTabs({ defaultValue: "upload" });

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

{#if data.url}
    <!-- svelte-ignore a11y_missing_attribute -->
    <img src={data.url} />
{/if}
<div class="flex flex-row gap-2">
    <button
        class="input-button"
        use:melt={$trigger}
        use:melt={$tab_trigger("upload")}>Upload</button
    >
    <button
        class="input-button"
        use:melt={$trigger}
        use:melt={$tab_trigger("select")}>Select Existing</button
    >
</div>
{#if additionalData.field_type.type !== "null"}
    <Field
        field={additionalData}
        bind:data={data["additional_data"]}
        ready={() => (innerReady = true)}
    ></Field>
{/if}
{#if $open}
    <div use:melt={$portalled}>
        <div
            use:melt={$overlay}
            class="fixed inset-0 z-50 backdrop-blur-sm backdrop-brightness-50"
            transition:fade={{ duration: 150 }}
        ></div>
        <div
            use:melt={$content}
            class="fixed z-50 left-1/2 top-1/2 max-h-[85vh] max-w-[90vw] flex
                        min-w-min -translate-x-1/2 -translate-y-1/2 p-8 bg-dark text-gray border-1 rounded-sm shadow-lg shadow-black transition-all"
            transition:flyAndScale={{
                duration: 150,
                y: 8,
                start: 0.96,
            }}
        >
            <div class="flex flex-col flex-initial" use:melt={$root}>
                <div use:melt={$list}>
                    <button use:melt={$tab_trigger("upload")}>Upload</button>
                    <button use:melt={$tab_trigger("select")}>Select</button>
                </div>
                <div use:melt={$tab_content("upload")}>
                    <div
                        use:filedrop={{ fileLimit: 1 }}
                        onfiledrop={(e) =>
                            uploadFile(e.detail.files.accepted[0])}
                        class="input-base w-sm h-16"
                    ></div>
                    <progress value={uploadProgress}> </progress>
                </div>
                <div
                    use:melt={$tab_content("select")}
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
                                        $open = false;
                                    }}
                                    aria-label="select image"
                                >
                                    <!-- svelte-ignore a11y_missing_attribute -->
                                    <img src={url} />
                                </button>
                            {/each}
                        </div>
                    {/await}
                </div>
            </div>
        </div>
    </div>
{/if}
