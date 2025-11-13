<script lang="ts">
    import type { EditorField } from "$ts/EditorField";
    import { onMount, type Snippet } from "svelte";
    import {
        now,
        getLocalTimeZone,
        toZoned,
        parseDate,
        parseAbsoluteToLocal,
        toCalendarDateTime,
        parseDateTime,
        CalendarDateTime,
        CalendarDate,
        type DateValue,
    } from "@internationalized/date";
    import { fly } from "svelte/transition";
    import { error } from "@sveltejs/kit";
    import { DatePicker, TimeField } from "bits-ui";
    import Label from "../Label.svelte";

    let {
        field,
        data = $bindable(),
        ready,
    }: { field: EditorField; data: any; ready: () => void } = $props();

    if (
        field.field_type.type != "date" &&
        field.field_type.type != "date-time"
    ) {
        error(500, "invalid type");
    }

    let granularity: "day" | "minute" =
        field.field_type.type == "date" ? "day" : "minute";

    let value: DateValue | undefined = $state();

    // "drac why don't you put this into the initial state????"
    // because it explodes. i don't know why. it just does.
    if (data && field.field_type.type == "date") {
        value = parseDate(data);
    } else if (data && field.field_type.type == "date-time") {
        value = toCalendarDateTime(parseAbsoluteToLocal(data));
    }

    const getValue = () => value;
    const setValue = (newValue: DateValue) => {
        value = newValue;
        if (newValue) {
            if (field.field_type.type == "date-time") {
                data = toZoned(newValue, getLocalTimeZone()).toAbsoluteString();
            }
            if (field.field_type.type == "date") {
                data = toZoned(newValue, getLocalTimeZone())
                    .toAbsoluteString()
                    .split("T", 1)[0];
            }
        }
    };

    // const {
    //     elements: {
    //         calendar,
    //         cell,
    //         content,
    //         field: ui_field,
    //         grid,
    //         heading,
    //         label,
    //         nextButton,
    //         prevButton,
    //         segment,
    //         trigger,
    //     },
    //     states: {
    //         months,
    //         headingValue,
    //         weekdays,
    //         segmentContents,
    //         value,
    //         open,
    //     },
    //     helpers: { isDateDisabled, isDateUnavailable },
    // } = createDatePicker({
    //     //defaultPlaceholder: toCalendarDateTime(now(getLocalTimeZone())),
    //     defaultValue: initial,
    //     granularity: field.field_type.type == "date" ? "day" : "second",
    //     forceVisible: true,
    // });

    // $effect(() => {
    //     if ($value) {
    //         console.log(field.field_type.type);
    //         if (field.field_type.type == "date-time") {
    //             data = toZoned($value, getLocalTimeZone()).toAbsoluteString();
    //         }
    //         if (field.field_type.type == "date") {
    //             data = toZoned($value, getLocalTimeZone())
    //                 .toAbsoluteString()
    //                 .split("T", 1)[0];
    //         }
    //     }
    // });

    onMount(() => {
        ready();
    });
</script>

<Label {field}>
    <DatePicker.Root {granularity} bind:value={getValue, setValue}>
        <DatePicker.Input class="flex input-base w-fit p-2 gap-1">
            {#snippet children({ segments })}
                {#each segments as { part, value }}
                    <DatePicker.Segment
                        class="focus-visible:outline-purple focus-visible:outline-solid"
                        {part}
                    >
                        {value}
                    </DatePicker.Segment>
                {/each}
                <DatePicker.Trigger
                    class="mx-2 input-button !p-0 !my-0 w-6 h-6 flex justify-center items-center"
                >
                    <div class="i-ph-calendar-blank"></div>
                </DatePicker.Trigger>
            {/snippet}
        </DatePicker.Input>
        <DatePicker.Content forceMount class="z-40">
            {#snippet child({ wrapperProps, props, open })}
                {#if open}
                    <div {...wrapperProps}>
                        <div
                            {...props}
                            transition:fly={{ y: 10, duration: 100 }}
                        >
                            <DatePicker.Calendar
                                class="bg-dark rounded-xs shadow-sm border-1 p-2 my-2"
                            >
                                {#snippet children({ months, weekdays })}
                                    <DatePicker.Header
                                        class="flex flex-row w-full"
                                    >
                                        <DatePicker.PrevButton>
                                            <div class="i-ph-caret-left"></div>
                                        </DatePicker.PrevButton>
                                        <DatePicker.Heading class="mx-auto" />
                                        <DatePicker.NextButton>
                                            <div class="i-ph-caret-right"></div>
                                        </DatePicker.NextButton>
                                    </DatePicker.Header>
                                    <div class="flex flex-col space-y-4 pt-4">
                                        {#each months as month}
                                            <DatePicker.Grid
                                                class="select-none space-y-1"
                                            >
                                                <DatePicker.GridHead>
                                                    <DatePicker.GridRow
                                                        class="mb-1 w-full grid grid-cols-7"
                                                    >
                                                        {#each weekdays as day}
                                                            <DatePicker.HeadCell
                                                            >
                                                                {day}
                                                            </DatePicker.HeadCell>
                                                        {/each}
                                                    </DatePicker.GridRow>
                                                </DatePicker.GridHead>
                                                <DatePicker.GridBody>
                                                    {#each month.weeks as weekDates}
                                                        <DatePicker.GridRow
                                                            class="grid grid-cols-7"
                                                        >
                                                            {#each weekDates as date}
                                                                <DatePicker.Cell
                                                                    class="p-2 inline-flex items-center justify-center"
                                                                    {date}
                                                                    month={month.value}
                                                                >
                                                                    <DatePicker.Day
                                                                    />
                                                                </DatePicker.Cell>
                                                            {/each}
                                                        </DatePicker.GridRow>
                                                    {/each}
                                                </DatePicker.GridBody>
                                            </DatePicker.Grid>
                                        {/each}
                                    </div>

                                    {#if field.field_type.type == "date-time"}
                                        <TimeField.Root
                                            bind:value={getValue, setValue}
                                            granularity="minute"
                                        >
                                            <TimeField.Input
                                                class="flex input-base w-fit p-2 gap-1"
                                            >
                                                {#snippet children({
                                                    segments,
                                                })}
                                                    {#each segments as { part, value }}
                                                        <TimeField.Segment
                                                            class="focus-visible:outline-purple focus-visible:outline-solid"
                                                            {part}
                                                        >
                                                            {value}
                                                        </TimeField.Segment>
                                                    {/each}
                                                {/snippet}
                                            </TimeField.Input>
                                        </TimeField.Root>
                                    {/if}
                                {/snippet}
                            </DatePicker.Calendar>
                        </div>
                    </div>
                {/if}
            {/snippet}
        </DatePicker.Content>
    </DatePicker.Root>
</Label>

<!-- <label for={field.name} use:melt={$label}>
    {field.title}
    {@render input(openButton)}
    {#if $open}
        <div
            use:melt={$content}
            transition:fly={{ y: 10, duration: 100 }}
            class="bg-dark rounded-sm shadow border-1 p-2 my-2"
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
                                            aria-disabled={$isDateDisabled(
                                                date,
                                            ) || $isDateUnavailable(date)}
                                        >
                                            <div
                                                class="flex items-center justify-center text-gray-200 data-[disabled]:opacity-40 w-6 h-6 hover:bg-gray"
                                                use:melt={$cell(
                                                    date,
                                                    month.value,
                                                )}
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

        </div>
    {/if}
</label> -->
