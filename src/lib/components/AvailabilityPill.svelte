<script lang="ts">
    interface Props {
        available: boolean;
        year: number;
    }

    let { available, year }: Props = $props();

    let isPulsing: boolean = $state(false);
    const animationDuration = 1000;

    if (available) {
        setInterval(() => {
            isPulsing = !isPulsing;
        }, animationDuration);
    }

    const pillClasses = $derived(
        available
            ? "bg-green-100 text-green-600 dark:bg-green-900 dark:text-green-300"
            : "bg-yellow-100 text-yellow-600 dark:bg-yellow-900 dark:text-yellow-200"
    );
    const dotClasses = $derived(
        available
            ? "bg-green-600 dark:bg-green-300"
            : "bg-yellow-600 dark:bg-yellow-200"
    );
</script>

<div
    class="inline-flex items-center px-4 py-2 rounded-full font-medium select-none relative {pillClasses}"
>
    <div class="inline-block w-2 h-2 mr-2 rounded-full relative {dotClasses}">
        {#if available && isPulsing}
            <div
                class="absolute w-2 h-2 rounded-full bg-black bg-opacity-40 dark:bg-white dark:bg-opacity-40 animate-ping"
                style="animation-duration: {animationDuration}ms; animation-iteration-count: 1;"
            ></div>
        {/if}
    </div>
    <span class="text-[0.95rem] md:text-base xl:text-[1.05rem]">
        {available ? "Open" : "Employed"} for Internship Position {year}
    </span>
</div>
