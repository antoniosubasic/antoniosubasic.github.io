<script lang="ts">
    import "../app.css";
    import logoLight from "$lib/assets/logo-light.svg";
    import logoDark from "$lib/assets/logo-dark.svg";

    let { children } = $props();
</script>

<svelte:head>
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
