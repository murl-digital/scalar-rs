<script lang="ts">
    import type { EditorField } from "scalar-types";
    import { error } from "@sveltejs/kit";
    import Label from "../Label.svelte";
    import { colord, type Colord } from "colord";
    import ColorPicker, { ChromeVariant } from "svelte-awesome-color-picker";
    import { onMount } from "svelte";
    import Wrapper from "./color/Wrapper.svelte";
    import PickerIndicator from "./color/PickerIndicator.svelte";
    import TextInput from "./color/TextInput.svelte";

    let {
        field,
        data = $bindable(),
        ready,
    }: { field: EditorField; data: any; ready: () => void } = $props();

    let isAlpha = $derived(
        field.field_type.type === "struct" &&
            field.field_type.fields.some((f) => f.name === "a"),
    );

    if (
        field.field_type.type !== "struct" ||
        field.field_type.component_key !== "color-input"
    ) {
        error(500, "ColorInput was not given a color field");
    }

    const convert = (object: {
        r: number;
        g: number;
        b: number;
        a?: number;
    }) => {
        return {
            r: object.r,
            g: object.g,
            b: object.b,
            a: object.a ? object.a / 255 : undefined,
        };
    };

    let rgba = $state(
        data
            ? colord(convert(data)).toRgb()
            : field?.field_type?.default
              ? colord(convert(field.field_type.default)).toRgb()
              : null,
    );

    $effect(() => {
        if (rgba) {
            data = isAlpha
                ? {
                      r: Math.floor(rgba.r),
                      g: Math.floor(rgba.g),
                      b: Math.floor(rgba.b),
                      a: Math.floor(255 * rgba.a),
                  }
                : {
                      r: Math.floor(rgba.r),
                      g: Math.floor(rgba.g),
                      b: Math.floor(rgba.b),
                  };
        }
    });

    onMount(() => {
        ready();
    });
</script>

<Label {field}>
    <ColorPicker
        bind:rgb={rgba}
        {isAlpha}
        components={{
            wrapper: Wrapper,
            pickerIndicator: PickerIndicator,
            textInput: TextInput,
        }}
        sliderDirection="vertical"
        isDialog={false}
    />
</Label>
