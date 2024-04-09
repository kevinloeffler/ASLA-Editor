<!-- logic is implemented in hooks.client -->

{#if darkMode}
    <img class="splash-image" src="/assets/images/loadingscreen/ASLA%20Editor%20loading%20dark.svg" alt="ASLA Editor">
{:else}
    <img class="splash-image" src="/assets/images/loadingscreen/ASLA%20Editor%20loading.svg" alt="ASLA Editor">
{/if}



<script>
    import {onMount} from 'svelte'

    let darkMode = false

    onMount(() => {
        darkMode = window.matchMedia('(prefers-color-scheme: dark)').matches
    })
</script>


<style>
    .splash-image {
        position: fixed;
        top: 0;
        right: 0;
        bottom: 0;
        left: 0;
        object-fit: cover;
    }
</style>
