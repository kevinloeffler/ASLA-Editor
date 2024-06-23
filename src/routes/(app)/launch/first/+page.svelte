<h1>Willkommen beim</h1>
<h1 class="titel">ASLA Editor</h1>

<div class="wrapper">
    <div class="col">
        <button class="panel col">
            <img src="/assets/icons/download-guide.svg" alt="Anleitung">
            Anleitung<br>Herunterladen
        </button>
    </div>
    <div class="col">
        <button class="panel" on:click={selectEnvironment}>
            <img src="/assets/icons/find-config.svg" alt="Öffnen">
            Umgebung<br>Öffnen
        </button>
        <p class="divider">oder</p>
        <button class="panel panel-right-bottom" on:click={() => {}}>
            <img src="/assets/icons/create-config.svg" alt="Erstellen">
            Umgebung<br>Erstellen
        </button>
    </div>
</div>


<script lang="ts">

    import {createDir, exists, writeTextFile} from '@tauri-apps/api/fs'
    import {appConfigDir, BaseDirectory} from '@tauri-apps/api/path'
    import {open} from '@tauri-apps/api/dialog'
    import {invoke} from '@tauri-apps/api/tauri'
    import {STATE} from '$lib/services/state-manager'

    async function selectEnvironment() {
        const env_path = await open({
            directory: false,
            multiple: false,
            defaultPath: '/',
        })
        if (typeof env_path !== 'string') return

        const success = await STATE.writeConfigFile(env_path)
        if (!success) return

        await invoke('hide_welcome_window')
        await STATE.reload()
    }

</script>


<style>

    h1 {
        font-weight: 800;
        font-size: 28pt;
        line-height: 28pt;
    }

    .titel {
        display: inline;
        background: linear-gradient(to right, var(--red), var(--orange));
        -webkit-background-clip: text;
        background-clip: text;
        color: transparent;
    }

    .wrapper {
        width: 100%;
        height: 100%;
        margin-top: 16px;
        min-height: 230px;
        display: flex;
        gap: 4px;
    }

    .col {
        width: 50%;
        display: flex;
        flex-direction: column;
        align-items: center;
    }

    .panel {
        width: 100%;
        height: 100%;
        background-color: var(--raised-color);
        border-radius: 8px;
        border: none;
        padding: 8px;
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        gap: 12px;
        font-weight: 500;
        cursor: pointer;
        transition-duration: 200ms;
    }

    .panel:hover {
        background-color: var(--raised-hover-color);
    }

    .panel-right-bottom {
        flex-direction: row;
        height: auto;
        min-height: 80px;
        text-align: left;
        margin-top: 4px;
    }

    .divider {
        /*position: absolute;*/
        display: inline;
        padding: 4px 16px;
        margin: -14px 0;
        background-color: var(--background-color);
        border-radius: 100px;
        z-index: 99;
    }

</style>
