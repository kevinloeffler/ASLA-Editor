
<BackButton label="Projekte" target="/projects" />
<h1>{code || '-'}</h1>


<div class="container">
    <div class="row">

        <div class="group">
            <label for="name">Name</label>
            <input bind:value={project.name} type="text" id="name" class="text-input">
            <p class="hint">Der Name des Projektes</p>
        </div>

        <div class="group">
            <label for="artefacts">Artefakte</label>
            <p>Hier werden Wörter und Textfragmente erfasst welche auf den Plänen dieses Projektes häufig
                auftreten
                und von der KI ignoriert werden sollten.</p>
            {#each project.artefacts as artefact}
                <div class="artefact">
                    <p>{artefact}</p>
                    <button on:click={() => deleteArtefact(artefact)}>
                        <img src="/assets/icons/delete-icon.svg" alt="Delete">
                    </button>
                </div>
            {/each}
            <div class="artefact">
                <input on:keydown={(event) => handleArtefactInput(event)} type="text" placeholder="+ Neues Artefakt">
            </div>
        </div>

    </div>
    <div class="row">

    </div>
</div>

<button on:click={saveProject} class="save-button">Speichern</button>


<script lang="ts">

    import {page} from '$app/stores'
    import BackButton from '$lib/components/BackButton.svelte'
    import {STATE} from '$lib/services/state-manager.js'
    import {onMount} from 'svelte'
    import {goto} from '$app/navigation'

    let code = $page.params.code
    let project: Project = {
        code: '',
        name: '',
        exportDirectory: '',
        workingDirectory: '',
        subfolders: false,
        artefacts: [],
    }

    onMount(() => {
        project = structuredClone(STATE.projects.get(code)) || project
    })

    function deleteArtefact(artefact: string) {
        project.artefacts = project.artefacts.filter( art => art !== artefact )
    }

    function handleArtefactInput(event: KeyboardEvent) {
        if (event.key !== 'Enter') return
        if (event.target.value === '') return
        project.artefacts = [...project.artefacts , event.target.value]
        event.target.value = ''
        console.log('new artefacts:', project.artefacts)
    }

    async function saveProject() {
        await STATE.projects.update(project)
        await goto('/projects')
    }

</script>


<style>

    .group {
        display: flex;
        flex-direction: column;
        max-width: 600px;
        padding: 16px 12px;
        background-color: var(--raised-color);
        border-radius: 8px;
        margin-bottom: 8px;
    }

    label {
        font-weight: 600;
        margin-bottom: 4px;
    }

    .hint {
        color: var(--foreground-color);
        margin-top: 4px;
        font-size: 9pt;
    }

    .artefact {
        display: flex;
        justify-content: space-between;
        background-color: var(--background-color);
        border-radius: 4px;
        padding: 8px 12px;
        margin-bottom: 4px;
    }

    .artefact > button {
        background-color: transparent;
        border: none;
        cursor: pointer;
    }

    .artefact > input {
        width: 100%;
        background-color: transparent;
        border: none;
        outline: none;
    }

    .text-input {
        padding: 8px 12px;
        background-color: var(--background-color);
        border-radius: 8px;
        border: none;
        outline: none;
    }

    .text-input:focus {
        outline: 2px solid var(--foreground-color);
    }

    .save-button {
        display: block;
        max-width: 600px;
        border: solid 2px var(--foreground-color);
        border-radius: 8px;
        text-align: center;
        color: var(--foreground-color);
        background-color: transparent;
        padding: 8px 0;
        font-weight: 600;
        width: 100%;
        transition-duration: 200ms;
    }

    .save-button:hover, .save-button:focus {
        color: var(--background-color);
        background-color: var(--foreground-color);
        cursor: pointer;
    }

</style>
