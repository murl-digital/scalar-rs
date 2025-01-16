<script lang="ts">
    import type { EditorField } from "$ts/EditorField";
    import { error } from "@sveltejs/kit";
    import { colord } from "colord";
    import type { Colord } from "colord";
    import { Spring } from "svelte/motion";

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

    let initialColor = data
        ? colord(data).toHsv()
        : field?.field_type?.default
          ? colord(field.field_type.default).toHsv()
          : null;

    let hue = $state(initialColor?.h ?? 0);
    let sat = $state(initialColor?.s ?? 0);
    let val = $state(initialColor?.v ?? 0);
    let alpha = $state(initialColor?.a ?? 1);

    let colorBlock: HTMLCanvasElement | undefined = $state();
    let colorStrip: HTMLCanvasElement | undefined = $state();
    let alphaStrip: HTMLCanvasElement | undefined = $state();

    let colorBlockCursor = Spring.of(
        () => {
            return {
                x: (sat / 100) * (colorBlock?.width ?? 0),
                y:
                    (colorBlock?.height ?? 0) -
                    (val / 100) * (colorBlock?.height ?? 0),
            };
        },
        { stiffness: 0.08, damping: 0.3 },
    );
    let colorStripCursor = Spring.of(
        () => {
            return { y: (hue / 360) * (colorStrip?.height ?? 0) };
        },
        { stiffness: 0.08, damping: 0.3 },
    );
    let alphaStripCursor = Spring.of(
        () => {
            return { y: (1 - alpha) * (colorStrip?.height ?? 0) };
        },
        { stiffness: 0.08, damping: 0.3 },
    );

    let color: Colord = $derived(colord({ h: hue, s: sat, v: val, a: alpha }));
    $effect(() => {
        if (!colorBlockDragging && !colorStripDragging && !alphaStripDragging) {
            let rgba = color.rgba;
            data = isAlpha
                ? {
                      r: Math.floor(rgba.r),
                      g: Math.floor(rgba.g),
                      b: Math.floor(rgba.b),
                      a: Math.floor(255 * alpha),
                  }
                : {
                      r: Math.floor(rgba.r),
                      g: Math.floor(rgba.g),
                      b: Math.floor(rgba.b),
                  };
        }
    });
    $effect(() => {
        ready();
    });

    $effect(() => {
        if (colorBlock) {
            const colorBlockCtx =
                colorBlock.getContext("2d") ??
                error(500, "couldn't initialize context");
            colorBlockCtx.reset();

            let color = colord({ h: hue, s: 255, v: 255 });

            let whiteGrd = colorBlockCtx.createLinearGradient(
                0,
                0,
                colorBlock.height,
                colorBlock.width,
            );
            whiteGrd.addColorStop(0, "white");
            whiteGrd.addColorStop(1, color.toHex());

            let blackGrd = colorBlockCtx.createLinearGradient(
                0,
                0,
                0,
                colorBlock.height,
            );
            blackGrd.addColorStop(0, "transparent");
            blackGrd.addColorStop(1, "black");

            colorBlockCtx.fillStyle = color.toHex();
            colorBlockCtx.fillRect(0, 0, colorBlock.width, colorBlock.height);
            colorBlockCtx.fillStyle = whiteGrd;
            colorBlockCtx.fillRect(0, 0, colorBlock.width, colorBlock.height);
            colorBlockCtx.fillStyle = blackGrd;
            colorBlockCtx.fillRect(0, 0, colorBlock.width, colorBlock.height);

            colorBlockCtx.beginPath();
            colorBlockCtx.lineWidth = 1;
            colorBlockCtx.strokeStyle = "white";
            colorBlockCtx.shadowColor = "black";
            colorBlockCtx.shadowBlur = 4;
            colorBlockCtx.arc(
                colorBlockCursor.current.x,
                colorBlockCursor.current.y,
                7,
                0,
                2 * Math.PI,
            );
            colorBlockCtx.stroke();
        }
    });

    $effect(() => {
        if (colorStrip) {
            const colorStripCtx =
                colorStrip?.getContext("2d") ??
                error(500, "couldn't initialize context");
            colorStripCtx.reset();

            let hueColors = colorStripCtx.createLinearGradient(
                0,
                0,
                0,
                colorStrip.height,
            );
            hueColors.addColorStop(0, "rgb(255, 0, 0)");
            hueColors.addColorStop(0.17, "rgb(255, 255, 0)");
            hueColors.addColorStop(0.33, "rgb(0, 255, 0)");
            hueColors.addColorStop(0.5, "rgb(0, 255, 255)");
            hueColors.addColorStop(0.67, "rgb(0, 0, 255)");
            hueColors.addColorStop(0.83, "rgb(255, 0, 255)");
            hueColors.addColorStop(1, "rgb(255, 0, 0)");
            colorStripCtx.fillStyle = hueColors;
            colorStripCtx.fillRect(0, 0, colorStrip.width, colorStrip.height);

            colorStripCtx.fillStyle = "white";
            colorStripCtx.shadowColor = "black";
            colorStripCtx.shadowBlur = 4;
            colorStripCtx.fillRect(
                0,
                colorStripCursor.current.y - 4,
                colorStrip.width,
                5,
            );
        }
    });

    $effect(() => {
        if (alphaStrip) {
            const alphaStripCtx =
                alphaStrip.getContext("2d") ??
                error(500, "couldn't initialize context");
            alphaStripCtx.reset();

            let cols = 2;
            let rows = 50;
            let squareSize = 5;

            let whiteSquareColor = "#BDBDBD";
            let blackSquareColor = "#707070";

            for (let j = 0; j * squareSize < alphaStrip.height; j++)
                for (let i = 0; i * squareSize < alphaStrip.width; i++) {
                    if (
                        (i % 2 == 0 && j % 2 == 0) ||
                        (i % 2 != 0 && j % 2 != 0)
                    )
                        alphaStripCtx.fillStyle = whiteSquareColor;
                    else alphaStripCtx.fillStyle = blackSquareColor;
                    alphaStripCtx.fillRect(
                        i * squareSize,
                        j * squareSize,
                        squareSize,
                        squareSize,
                    );
                }

            let blackGrd = alphaStripCtx.createLinearGradient(
                0,
                0,
                0,
                alphaStrip.height,
            );
            let opaqueColor = color.alpha(1);
            blackGrd.addColorStop(0, opaqueColor.toHex());
            blackGrd.addColorStop(1, "transparent");

            alphaStripCtx.fillStyle = blackGrd;
            alphaStripCtx.fillRect(0, 0, alphaStrip.width, alphaStrip.height);

            alphaStripCtx.fillStyle = "white";
            alphaStripCtx.shadowColor = "black";
            alphaStripCtx.shadowBlur = 4;
            alphaStripCtx.fillRect(
                0,
                alphaStripCursor.current.y - 4,
                alphaStrip.width,
                5,
            );
        }
    });

    let colorBlockDragging = $state(false);
    let colorStripDragging = $state(false);
    let alphaStripDragging = $state(false);

    const clamp = (n: number, min: number, max: number) =>
        Math.min(Math.max(n, min), max);

    function handleCanvasCursor(e: MouseEvent) {
        if (colorBlock) {
            let rec = colorBlock.getBoundingClientRect();
            let x = clamp(e.clientX - rec.left, 0, rec.width);
            let y = clamp(e.clientY - rec.top, 0, rec.height);

            sat = Math.round((x / colorBlock.width) * 100);
            val = Math.round(100 - (y / colorBlock.height) * 100);
        }
    }

    function handleHueCursor(e: MouseEvent) {
        if (colorStrip) {
            var rec = colorStrip.getBoundingClientRect();
            var x = e.clientX - rec.left;
            var y = e.clientY - rec.top;

            hue = Math.round(Math.abs((y / colorStrip.height) * 360) % 360);
        }
    }

    function handleAlphaCursor(e: MouseEvent) {
        if (alphaStrip) {
            var rec = alphaStrip.getBoundingClientRect();
            var x = e.clientX - rec.left;
            var y = e.clientY - rec.top;

            alpha = 1 - Math.min(Math.max(y / alphaStrip.height, 0), 1);
        }
    }

    function colorBlockMouseDown(e: MouseEvent) {
        colorBlockDragging = true;
        handleCanvasCursor(e);
    }

    function colorStripMouseDown(e: MouseEvent) {
        colorStripDragging = true;
        handleHueCursor(e);
    }

    function alphaStripMouseDown(e: MouseEvent) {
        alphaStripDragging = true;
        handleAlphaCursor(e);
    }
</script>

<svelte:window
    onmouseup={() => {
        colorBlockDragging = colorStripDragging = alphaStripDragging = false;
    }}
    onmousemove={(e) => {
        if (colorBlockDragging) {
            handleCanvasCursor(e);
        }
        if (colorStripDragging) {
            handleHueCursor(e);
        }
        if (alphaStripDragging) {
            handleAlphaCursor(e);
        }
    }}
/>

<div class="flex gap-3">
    <canvas
        width="256"
        height="256"
        onmousedown={colorBlockMouseDown}
        bind:this={colorBlock}
    ></canvas>
    <canvas
        onmousedown={colorStripMouseDown}
        bind:this={colorStrip}
        width="20"
        height="256"
    >
    </canvas>
    {#if isAlpha}
        <canvas
            onmousedown={alphaStripMouseDown}
            bind:this={alphaStrip}
            width="20"
            height="256"
        >
        </canvas>
    {/if}
    <div class="w-12 h-6" style:background={color.toRgbString()}></div>
</div>
