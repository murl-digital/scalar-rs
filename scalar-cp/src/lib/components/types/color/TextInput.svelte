<script lang="ts">
    import type { RgbaColor, HsvaColor } from "colord";

    interface Props {
        /** if set to false, disables the alpha channel */
        isAlpha: boolean;
        /** rgb color */
        rgb: RgbaColor;
        /** hsv color */
        hsv: HsvaColor;
        /** hex color */
        hex: string;
        /** configure which hex, rgb and hsv inputs will be visible and in which order. If overridden, it is necessary to provide at least one value */
        textInputModes: Array<"hex" | "rgb" | "hsv">;
        /** all translation tokens used in the library; can be partially overridden; see [full object type](https://github.com/Ennoriel/svelte-awesome-color-picker/blob/master/src/lib/utils/texts.ts) */
        texts: {
            label: {
                r: string;
                g: string;
                b: string;
                h: string;
                s: string;
                v: string;
                a: string;
                hex: string;
            };
            color: {
                rgb: string;
                hsv: string;
                hex: string;
            };
            changeTo: string;
        };
        /** listener, dispatch an event when one of the color changes */
        onInput: (color: {
            hsv?: HsvaColor;
            rgb?: RgbaColor;
            hex?: string;
        }) => void;
    }

    let {
        isAlpha,
        rgb = $bindable(),
        hsv = $bindable(),
        hex = $bindable(),
        textInputModes,
        texts,
        onInput,
    }: Props = $props();

    const HEX_COLOR_REGEX = /^#?([A-F0-9]{6}|[A-F0-9]{8})$/i;

    let mode: "hex" | "rgb" | "hsv" = $state(textInputModes[0] || "hex");

    let nextMode = $derived(
        textInputModes[
            (textInputModes.indexOf(mode) + 1) % textInputModes.length
        ],
    );
    let prevMode = $derived(
        textInputModes[
            (textInputModes.indexOf(mode) - 1 == -1
                ? textInputModes.length - 1
                : textInputModes.indexOf(mode) - 1) % textInputModes.length
        ],
    );

    let h = $derived(Math.round(hsv.h));
    let s = $derived(Math.round(hsv.s));
    let v = $derived(Math.round(hsv.v));
    let a = $derived(hsv.a === undefined ? 1 : Math.round(hsv.a * 100) / 100);

    type InputEvent = Event & { currentTarget: EventTarget & HTMLInputElement };

    function updateHex(e: InputEvent) {
        const target = e.target as HTMLInputElement;
        if (HEX_COLOR_REGEX.test(target.value)) {
            hex = target.value;
            onInput({ hex });
        }
    }

    function updateRgb(property: string) {
        return function (e: InputEvent) {
            let value = parseFloat((e.target as HTMLInputElement).value);
            rgb = { ...rgb, [property]: isNaN(value) ? 0 : value };
            onInput({ rgb });
        };
    }

    function updateHsv(property: string) {
        return function (e: InputEvent) {
            let value = parseFloat((e.target as HTMLInputElement).value);
            hsv = { ...hsv, [property]: isNaN(value) ? 0 : value };
            onInput({ hsv });
        };
    }
</script>

<div class="flex flex-col gap-2 p-2">
    <div class="flex flex-row gap-2">
        {#if mode === "hex"}
            <input
                class="input-base"
                aria-label={texts.label.hex}
                value={hex}
                oninput={updateHex}
            />
        {:else if mode === "rgb"}
            <input
                class="input-base w-1/3"
                aria-label={texts.label.r}
                value={rgb.r}
                type="number"
                min="0"
                max="255"
                oninput={updateRgb("r")}
            />
            <input
                class="input-base w-1/3"
                aria-label={texts.label.g}
                value={rgb.g}
                type="number"
                min="0"
                max="255"
                oninput={updateRgb("g")}
            />
            <input
                class="input-base w-1/3"
                aria-label={texts.label.b}
                value={rgb.b}
                type="number"
                min="0"
                max="255"
                oninput={updateRgb("b")}
            />
        {:else}
            <input
                class="input-base w-1/3"
                aria-label={texts.label.h}
                value={h}
                type="number"
                min="0"
                max="360"
                oninput={updateHsv("h")}
            />
            <input
                class="input-base w-1/3"
                aria-label={texts.label.s}
                value={s}
                type="number"
                min="0"
                max="100"
                oninput={updateHsv("s")}
            />
            <input
                class="input-base w-1/3"
                aria-label={texts.label.v}
                value={v}
                type="number"
                min="0"
                max="100"
                oninput={updateHsv("v")}
            />
        {/if}
    </div>
    {#if isAlpha}
        <input
            class="input-base grow p-2 appearance-none"
            aria-label={texts.label.a}
            value={a}
            type="number"
            min="0"
            max="1"
            step="0.01"
            oninput={mode === "hsv" ? updateHsv("a") : updateRgb("a")}
        />
    {/if}

    {#if textInputModes.length > 1}
        <div class="flex items-center mx-auto">
            <button
                class="input-button"
                onclick={() => (mode = prevMode)}
                title={`${texts.changeTo}${texts.color[prevMode]}`}
            >
                <div class="i-ph-caret-left"></div>
            </button>
            <div class="">
                {texts.color[mode]}
            </div>
            <button
                class="input-button"
                onclick={() => (mode = nextMode)}
                title={`${texts.changeTo}${texts.color[nextMode]}`}
            >
                <div class="i-ph-caret-right"></div>
            </button>
        </div>
    {:else}
        <div class="input-button">{texts.color[mode]}</div>
    {/if}
</div>
