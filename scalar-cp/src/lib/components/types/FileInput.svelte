<script lang="ts">
    import type { EditorField } from "scalar-types";
    import { error } from "@sveltejs/kit";
    import Label from "../Label.svelte";
    import { filedrop } from "filedrop-svelte";
    import { base } from "$app/paths";
    import { wire } from "$lib/api";
    import type { FileWithPath } from "file-selector";

    let {
        field,
        data = $bindable(),
        ready,
    }: { field: EditorField; data: any; ready: () => void } = $props();

    let innerReady = $state(false);

    if (
        field.field_type.type !== "struct" ||
        field.field_type.component_key !== "file"
    ) {
        error(500, "FileInput was not given a file field");
    }

    let additionalData = field.field_type.fields.find(
        (elem) => elem.name === "additional_data",
    );

    if (!additionalData) {
        error(500, "FieldInput was not given an additional_data field");
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

    async function uploadFile(f: FileWithPath) {
        const xhr = new XMLHttpRequest();
        const success = await new Promise((resolve) => {
            xhr.open("PUT", `${base}/api/files/upload`, true);
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

            const formData = new FormData();
            formData.append("file", f, f.name);

            xhr.send(formData);
        });
        console.log(success);
    }
</script>

<Label {field}>
    <div
        use:filedrop={{ fileLimit: 1 }}
        onfiledrop={(e) => uploadFile(e.detail.files.accepted[0])}
        class="input-base w-sm h-16"
    ></div>
</Label>
