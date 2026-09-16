<script lang="ts">
    import "@fontsource-variable/inter/opsz.css";
    import "../app.css";
    import interLatin from "@fontsource-variable/inter/files/inter-latin-opsz-normal.woff2?url";
    import logoLight from "$lib/assets/logo-light.svg";
    import logoDark from "$lib/assets/logo-dark.svg";

    let { children } = $props();
</script>

<svelte:head>
    <link
        rel="preload"
        href={interLatin}
        as="font"
        type="font/woff2"
        crossorigin="anonymous"
    />
    <link
        rel="icon"
        type="image/svg+xml"
        href={logoLight}
        media="(prefers-color-scheme: light)"
        sizes="any"
    />
    <link
        rel="icon"
        type="image/svg+xml"
        href={logoDark}
        media="(prefers-color-scheme: dark)"
        sizes="any"
    />
</svelte:head>

{@render children?.()}
