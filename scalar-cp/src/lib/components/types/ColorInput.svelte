<script lang="ts">
    import type { EditorField } from "$ts/EditorField";
    import { error } from "@sveltejs/kit";
    import { Colord, colord } from "colord";
    import { Spring } from "svelte/motion";

    // let {
    //     field,
    //     data = $bindable(),
    //     ready,
    // }: { field: EditorField; data: any; ready: () => void } = $props();

    let hue = $state(194);
    let sat = $state(100);
    let val = $state(100);

    let colorBlock: HTMLCanvasElement | undefined = $state();
    let colorStrip: HTMLCanvasElement | undefined = $state();

    let colorBlockCursor = Spring.of(
        () => {
            return {
                x: (sat / 100) * (colorBlock?.width ?? 0),
                y: 256 - (val / 100) * (colorBlock?.height ?? 0),
            };
        },
        { stiffness: 0.08, damping: 0.3 },
    );
    let colorStripCursor = Spring.of(
        () => {
            return { x: (hue / 360) * 360 };
        },
        { stiffness: 0.08, damping: 0.3 },
    );

    let color: Colord = $derived(colord({ h: hue, s: sat, v: val }));

    let colorBlockCtx: CanvasRenderingContext2D;
    let colorStripCtx: CanvasRenderingContext2D;

    $effect(() => {
        if (!colorBlockCtx) {
            colorBlockCtx =
                colorBlock?.getContext("2d") ??
                error(500, "couldn't initialize context");
        }

        let color = colord({ h: hue, s: 255, v: 255 });

        //colorBlockCtx.clearRect(0, 0, colorBlock.width, colorBlock.height);
        if (colorBlock) {
            colorBlockCtx.reset();

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
        if (!colorStripCtx) {
            colorStripCtx =
                colorStrip?.getContext("2d") ??
                error(500, "couldn't initialize context");
        }

        if (colorStrip) {
            colorStripCtx.reset();

            let hueColors = colorStripCtx.createLinearGradient(
                0,
                0,
                colorStrip.width,
                0,
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
                colorStripCursor.current.x - 4,
                0,
                5,
                colorStrip.height,
            );
        }
    });

    let colorBlockDragging = false;
    let colorStripDragging = false;

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

            hue = Math.round((x / colorStrip.width) * 360);
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
</script>

<svelte:window
    onmouseup={() => {
        colorBlockDragging = false;
        colorStripDragging = false;
    }}
    onmousemove={(e) => {
        if (colorBlockDragging) {
            handleCanvasCursor(e);
        }
        if (colorStripDragging) {
            handleHueCursor(e);
        }
    }}
/>

<canvas
    onmousedown={colorBlockMouseDown}
    bind:this={colorBlock}
    width="256"
    height="256"
></canvas>
<canvas
    onmousedown={colorStripMouseDown}
    bind:this={colorStrip}
    width="360"
    height="20"
>
</canvas>
<div class="w-12 h-6" style:background={color.toRgbString()}></div>
