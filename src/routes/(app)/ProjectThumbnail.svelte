<a href="/editor/{project.code}" class="wrapper">
    <p class="code">{project.code}</p>
    <p>{project.name}</p>
    <p>xyz Bilder</p>

    <a href="/projects/{project.code}" class="settings">
        <img src="/assets/icons/settings_icon_white.svg" alt="Projekteinstellungen">
    </a>
</a>


<script lang="ts">

    export let project: Project

</script>


<style>

    .wrapper {
        position: relative;
        display: block;
        background-color: var(--raised-color);
        border-radius: 8px;
        padding: 12px;
        transition-duration: 200ms;
    }

    .wrapper:hover, .wrapper:focus {
        background-color: var(--raised-hover-color);
    }

    .code {
        font-size: 26px;
        font-weight: bold;
        margin-bottom: 12px;
    }

    .settings {
        position: absolute;
        top: 12px;
        right: 12px;
        padding: 4px;
        background-color: var(--foreground-color);
        border-radius: 8px;
        transition-duration: 200ms;
    }

    .settings:hover {
        background-color: var(--text-color);
    }

    .wrapper:has(.settings:hover) {
        background-color: var(--raised-color);
    }

</style>
