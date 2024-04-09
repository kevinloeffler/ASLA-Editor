
<BackButton label="Abbrechen" target="/projects" />
<h1>Neues Projekt erstellen</h1>

<div class="group">
    <label for="name">Name</label>
    <input bind:value={name} type="text" id="name" class="text-input">
    <p class="hint">Der Name des Projektes</p>
</div>

<div class="group">
    <label for="code">Code</label>
    <input bind:value={code} type="text" id="code" class="text-input">
    <p class="hint">Der Projektcode wird verwendet um alle Bilder dem Projekt zuzuordnen</p>
</div>

<div class="artefacts-list">

    {#each artefacts as artefact}
        <div>{artefact}</div>
    {/each}

</div>

<div class="group">
    <label for="wd">Arbeitsverzeichnis</label>
    <button id="wd" class="button-input" on:click={async () => {workingDirectory = await selectDirectory(workingDirectory)}}>
        {workingDirectory || 'Auswählen'}
    </button>
</div>

<div class="group">
    <label for="ed">Exportverzeichnis</label>
    <button id="ed" class="button-input" on:click={async () => {exportDirectory = await selectDirectory(exportDirectory)}}>
        {exportDirectory || 'Auswählen'}
    </button>
</div>

<div class="group">
    <label for="structure">Struktur</label>
    <div class="checkbox-wrapper">
        <input bind:value={containsSubDir} type="checkbox" id="structure" class="checkbox-input">
        <label for="structure" class="checkbox-label">Projekt enthält Unterordner</label>
    </div>
</div>

<button class="save-button" on:click={saveProject}>Projekt speichern</button>

<script lang="ts">
    import BackButton from '$lib/components/BackButton.svelte'
    import {open} from '@tauri-apps/api/dialog'
    import {STATE} from '$lib/services/state-manager.js'
    import {goto} from '$app/navigation'
    import {ALERTS} from '$lib/services/alert-manager.js'

    let artefacts: any[] = []

    let code: Optional<string>
    let name: Optional<string>
    let workingDirectory: Optional<string>
    let exportDirectory: Optional<string>
    let containsSubDir = false

    async function selectDirectory(defaultDirectory: Optional<string>): Promise<Optional<string>> {
        let selected = await open({
            directory: true,
            multiple: false,
        })

        if (selected === null) {
            console.warn('No directory selected')
            return undefined
        } else if (Array.isArray(selected)) { selected = selected[0] }

        return selected
    }

    async function saveProject() {
        // minimal input validation
        if (!code || code.length < 3) { document.querySelector('#code')?.focus(); return }
        if (!name || name.length === 0) { document.querySelector('#name')?.focus(); return }
        if (!workingDirectory) { document.querySelector('#wd')?.focus(); return }
        if (!exportDirectory) { document.querySelector('#ed')?.focus(); return }

        await writeProjectToEnvironment()
        ALERTS.new('Projekt erfolgreich erstellt')
        await goto('/projects')
    }

    async function writeProjectToEnvironment() {
        await STATE.projects.add({
            code: code as string,
            name: name as string,
            workingDirectory: workingDirectory as string,
            exportDirectory: exportDirectory as string,
            subfolders: containsSubDir,
            artefacts: artefacts,
        })
    }


</script>


<style>

    h1 {
        margin-bottom: 12px;
    }

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

    .text-input {
        padding: 8px 12px;
        background-color: var(--background-color);
        border-radius: 8px;
        border: none;
        outline: none;
    }

    .text-input:focus, .button-input:focus, .checkbox-input:focus {
        outline: 2px solid var(--foreground-color);
    }

    .button-input {
        padding: 8px 12px;
        background-color: var(--background-color);
        text-align: left;
        border: none;
        outline: none;
        border-radius: 8px;
        cursor: pointer;
    }

    .checkbox-wrapper {
        display: flex;
        align-items: center;
    }

    .checkbox-input {
        width: 16px;
        height: 16px;
        border-radius: 4px;
    }

    .checkbox-label {
        font-weight: normal;
        margin: 0 0 0 8px;
    }

    .save-button {
        display: block;
        max-width: 600px;
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

    .save-button:hover, .save-button:focus {
        color: var(--background-color);
        background-color: var(--green);
        cursor: pointer;
    }

</style>
