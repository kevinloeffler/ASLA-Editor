<BackButton label="Startseite" target="/"/>
<h1>Projekte</h1>

<div class="project-list">
    <p class="code">Code</p>
    <p class="project">Projekt</p>
    <p class="progress">Fortschritt</p>
</div>

<!--<button on:click={() => window.location.href = '/projects/kla'}>goto KLA</button>-->

{#each projects as project}
    <ProjectListItem code="{project.code}" project="{project.name}" imagesDone="{92}" imagesTotal="{120}"/>
{/each}

<a href="/projects/new" class="new-project-button">Neues Projekt erstellen</a>


<script lang="ts">
    import BackButton from '$lib/components/BackButton.svelte'
    import ProjectListItem from './ProjectListItem.svelte'
    import {onMount} from 'svelte'
    import {STATE} from '$lib/services/state-manager.js'

    let projects: Project[] = []

    onMount(() => {
        projects = STATE.projects.all()
    })

</script>


<style>

    .project-list {
        display: flex;
        padding: 8px;
    }

    .code {
        width: 50px;
    }

    .project {
        width: 60%;
        max-width: 260px;
    }

    .new-project-button {
        display: block;
        border: solid 2px var(--green);
        border-radius: 8px;
        text-align: center;
        color: var(--green);
        background-color: transparent;
        padding: 8px 0;
        font-weight: 600;
        width: 100%;
        transition-duration: 200ms;
    }

    .new-project-button:hover {
        color: var(--background-color);
        background-color: var(--green);
        cursor: pointer;
    }

</style>
