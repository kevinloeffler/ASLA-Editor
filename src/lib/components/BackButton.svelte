<a href="{target}" class="button">
    <img src="/assets/icons/back%20button%20icon.svg" alt="<">
    {label}
</a>


<script lang="ts">

    export let label: string
    export let target: string

</script>


<style>

    .button {
        display: inline-flex;
        justify-content: center;
        align-items: center;
        padding: 6px 10px;
        font-size: 9pt;
        font-weight: 600;
        color: var(--foreground-color);
        background-color: var(--raised-color);
        outline: none;
        border: none;
        border-radius: 8px;
        transition-duration: 200ms;
    }

    .button:hover {
        background-color: var(--raised-hover-color);
    }

    .button > img {
        margin-right: 4px;
    }

</style>
