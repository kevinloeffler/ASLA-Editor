<h1>Willkommen beim ASLA Editor</h1>

<p>Es wurde keine Umgebung gefunden. Bitte wähle eine aus wie in der Anleitung beschrieben.</p>
<button on:click={() => { loadConfigFile() }}>File auswählen</button>

<p>Neue Umgebung erstellen. Achtung: damit wird eine neue Umgebung gestartet ohne bestehende Projekte.</p>
<button on:click={() => { createConfigFile() }}>Neues File erstellen</button>


<script lang="ts">

    import {createDir, exists, writeTextFile} from '@tauri-apps/api/fs'
    import {appConfigDir, BaseDirectory} from '@tauri-apps/api/path'
    import {open} from '@tauri-apps/api/dialog'
    import {invoke} from '@tauri-apps/api/tauri'

    async function loadConfigFile() {
        const env_path = await open({
            directory: false,
            multiple: false,
            defaultPath: '/',
        })
        if (typeof env_path !== 'string') return
        await writeConfigFile(env_path)
        await invoke('hide_welcome_window')
    }

    async function createConfigFile() {
        const env_path = await open({
            directory: true,
            multiple: false,
            defaultPath: '/',
        })
        if (typeof env_path !== 'string') return
        await writeConfigFile(env_path)
        await invoke('hide_welcome_window')
        return
    }

    async function writeConfigFile(environment: string) {
        const default_config = {
            env: environment
        }
        // create app config directory
        const configDirPath = await appConfigDir()
        console.log('config dir path:', configDirPath)
        if (! await exists(configDirPath)) { await createDir(configDirPath) }
        // write config file
        await writeTextFile('app.config', JSON.stringify(default_config), {dir: BaseDirectory.AppConfig})
    }


</script>
