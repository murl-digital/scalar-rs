<script lang="ts">
    import type { EditorField } from "$ts/EditorField";
    import { createDatePicker, melt } from "@melt-ui/svelte";
    import { onMount, type Snippet } from "svelte";

    const {
        elements: {
            calendar,
            cell,
            content,
            field: ui_field,
            grid,
            heading,
            label,
            nextButton,
            prevButton,
            segment,
            trigger,
        },
        states: { months, headingValue, weekdays, segmentContents, value },
        helpers: { isDateDisabled, isDateUnavailable },
    } = createDatePicker({
        granularity: "minute",
    });

    let {
        field,
        data = $bindable(),
        ready,
    }: { field: EditorField; data: any; ready: () => void } = $props();

    $inspect($value);
    $effect(() => {
        if ($value) {
            data = $value;
        }
    });

    onMount(() => {
        ready();
    });
</script>

{#snippet input(button?: Snippet)}
    <div
        class="flex input-base w-fit p-2 gap-1"
        id={field.name}
        use:melt={$ui_field}
    >
        {#each $segmentContents as seg}
            <div
                class="focus-visible:outline-purple focus-visible:outline-solid"
                use:melt={$segment(seg.part)}
            >
                {seg.value}
            </div>
        {/each}
        {@render button?.()}
    </div>
{/snippet}

{#snippet openButton()}
    <button
        class="mx-2 hover:bg-white hover:text-black transition-colors w-6 h-6 flex justify-center items-center focus-visible:outline-purple focus-visible:outline-solid"
        aria-label="Open Calendar"
        use:melt={$trigger}
    >
        <div class="i-ph-calendar-blank"></div>
    </button>
{/snippet}

<label for={field.name} use:melt={$label}>
    {field.title}
    {@render input(openButton)}
    <div
        use:melt={$content}
        class="backdrop-blur-sm bg-neutral-800 bg-opacity-40"
    >
        <div use:melt={$calendar}>
            <header class="flex flex-row">
                <button
                    class="bg-transparent color-gray"
                    aria-label="Previous Month"
                    use:melt={$prevButton}
                >
                    <div class="i-ph-arrow-left"></div>
                </button>
                <div use:melt={$heading}>
                    {$headingValue}
                </div>
                <button aria-label="Next Month" use:melt={$nextButton}>
                    <div class="i-ph-arrow-right"></div>
                </button>
            </header>
            {#each $months as month}
                <table class="w-full" use:melt={$grid}>
                    <thead aria-hidden="true">
                        <tr>
                            {#each $weekdays as day}
                                <th
                                    class="text-sm font-medium text-gray-300 uppercase tracking-wide w-6 h-6"
                                >
                                    {day}
                                </th>
                            {/each}
                        </tr>
                    </thead>
                    <tbody class="text-sm hover:cursor-pointer">
                        {#each month.weeks as days}
                            <tr>
                                {#each days as date}
                                    <td
                                        role="gridcell"
                                        aria-disabled={$isDateDisabled(date) ||
                                            $isDateUnavailable(date)}
                                    >
                                        <div
                                            class="flex items-center justify-center text-gray-200 data-[disabled]:opacity-40 w-6 h-6 hover:bg-gray"
                                            use:melt={$cell(date, month.value)}
                                        >
                                            {date.day}
                                        </div>
                                    </td>
                                {/each}
                            </tr>
                        {/each}
                    </tbody>
                </table>
            {/each}
        </div>
        {@render input()}
    </div>
</label>
