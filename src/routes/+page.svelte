<script lang="ts">
    import { AvailabilityPill, Taskbar } from "$lib";
</script>

<main
    class="max-w-[23.15rem] sm:max-w-xl md:max-w-3xl xl:max-w-7xl flex flex-col gap-8 items-center justify-center"
>
    <AvailabilityPill available={true} year={2026} />
    <h1
        class="text-[2.5rem] sm:text-[2.8rem] md:text-5xl xl:text-6xl text-center font-medium leading-[1.1]"
    >
        I'm <span class="underline-gradient">Antonio</span>, computer science
        student based in
        <span class="text-gradient">Austria</span>
    </h1>
    <p
        class="sm:text-md xl:text-lg max-w-[20rem] sm:max-w-lg md:max-w-2xl xl:max-w-6xl text-center text-stone-600 dark:text-stone-400"
    >
        A passionate computer science student specializing in software
        development and project management, with a focus on building efficient,
        scalable applications and crafting innovative digital solutions.
    </p>
    <Taskbar />
</main>
