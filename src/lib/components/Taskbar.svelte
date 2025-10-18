<script lang="ts">
    import linkedinIcon from "$lib/assets/linkedin.svg";
    import githubIcon from "$lib/assets/github.svg";
    import emailIcon from "$lib/assets/email.svg";

    const socials = [
        {
            name: "LinkedIn",
            url: "https://linkedin.com/in/subasicantonio",
            icon: linkedinIcon,
        },
        {
            name: "GitHub",
            url: "https://github.com/antoniosubasic",
            icon: githubIcon,
        },
        {
            name: "Email",
            url: "mailto:antonio.subasic.public@gmail.com",
            icon: emailIcon,
        },
    ];
</script>

<div
    class="taskbar bg-white dark:bg-black text-black dark:text-white fixed bottom-5 left-0 z-10 w-full flex justify-center p-3 md:p-4"
>
    <div
        class="border rounded-full border-neutral-400 dark:border-neutral-600 flex items-center justify-center gap-8 md:gap-10 p-4 px-6"
    >
        {#each socials as social}
            <a
                href={social.url}
                target="_blank"
                rel="noopener noreferrer"
                class="hover:opacity-70 transition-opacity"
            >
                <img
                    src={social.icon}
                    alt="{social.name} icon"
                    class="w-7 h-7 xl:w-8 xl:h-8 dark:invert"
                />
            </a>
        {/each}
    </div>
</div>
