{#if hasEnvironment}
<main>

    <div class="projects">
        <h2>Projekte:</h2>
        <div class="projects-list">
            {#each projects || [] as project }
                <ProjectThumbnail project="{project}" />
            {/each}
        </div>
        <button on:click={() => location.href = '/projects/new'} class="btn-compact new-project-btn">
            Neues Projekt erstellen
        </button>
    </div>

    <div class="row">
        <h2>&nbsp;</h2>
        <div class="upload">
            <Upload />
        </div>

        <div class="settings">
            <h3>Einstellungen</h3>
            <button on:click={() => window.location.href='/settings'} class="btn-compact">Öffnen</button>
        </div>
    </div>

</main>
{:else}
    <p class="no-env-warning">Keine Umgebung ausgewählt</p>
    <button on:click={() => STATE.reload()} class="btn-compact">Neu Laden</button>
{/if}


<script lang="ts">
    import ProjectThumbnail from './ProjectThumbnail.svelte'
    import {STATE} from '$lib/services/state-manager'
    import Upload from './projects/Upload.svelte'

    let projects: Project[]
    STATE._projects.subscribe(value => projects = value)

    let hasEnvironment: boolean
    STATE.hasEnvironment.subscribe(value => hasEnvironment = value)

</script>


<style>

    main {
        display: flex;
        flex-wrap: wrap;
        gap: 20px;
    }

    .projects {
        min-width: 220px;
        max-width: 900px;
        flex-grow: 1.5;
    }

    .row {
        min-width: 220px;
        max-width: 380px;
        flex-grow: 1;
    }

    .upload {
        min-width: 250px;
        background-color: var(--raised-color);
        border-radius: 8px;
        margin-bottom: 20px;
        overflow: hidden;
    }

    .settings {
        display: flex;
        justify-content: space-between;
        align-items: center;
        min-width: 250px;
        background-color: var(--raised-color);
        border-radius: 8px;
        padding: 20px;
    }

    .projects-list {
        display: grid;
        grid-template-columns: repeat(auto-fill, minmax(220px, 1fr));
        gap: 8px;
    }

    .new-project-btn {
        margin-top: 8px;
    }

</style>
