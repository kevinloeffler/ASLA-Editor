{#if image_name}
    <Editor
            imagePath="{project?.workingDirectory}/{image_name}"
            project="{project}"
            on:handle_image_load_error={async () => {image_name = await loadCurrentImage(true)} }
            on:next_image={async () => {image_name = await getNextImage(true)}}
            on:previous_image={async () => {image_name = await getNextImage(false)}}
    />
{:else}
    <div class="error-wrapper">
        <div>
            <p class="error-message">Keine Bilder im Arbeitsverzeichnis</p>
            <p class="error-working-directory">{project?.workingDirectory}</p>
            <BackButton label="Zurück" target="/"/>
        </div>
    </div>
{/if}


<script lang="ts">
    import {page} from '$app/stores'
    import {STATE} from '$lib/services/state-manager'
    import {invoke} from '@tauri-apps/api/tauri'
    import {onMount} from 'svelte'
    import Editor from '$lib/components/Editor.svelte'
    import BackButton from '$lib/components/BackButton.svelte'


    let code = $page.params.code
    let project = STATE.projects.get(code)

    let image_name: Optional<string>

    async function getNextImage(forward = true): Promise<Optional<string>> {
        try {
            const next_image: string = await invoke('get_next_image', {
                directory: `${project?.workingDirectory}`,
                currentImage: image_name,
                forward: forward,
            })
            console.log('next_image:', next_image)
            return next_image
        } catch (err) {
            console.error(err)
        }
    }

    async function loadCurrentImage(forceReload = false): Promise<Optional<string>> {
        try {
            const image: string = await invoke('read_current_file', {
                directory: `${project?.workingDirectory}`,
                reload: forceReload
            })
            return image === 'nil' ? undefined : image
        } catch (err) {
            console.error(err)
        }
    }

    async function updateCurrentImage(currentImage: string) {
        try {
            await invoke('update_current_file', {directory: `${project?.workingDirectory}`, content: currentImage})
        } catch (error) {
            console.error('Failed to update file:', error);
        }
    }

    onMount(async () => {
        image_name = await loadCurrentImage()
    })

</script>


<style>

    .error-wrapper {
        display: flex;
        flex-direction: column;
        justify-content: center;
        align-items: center;
        width: 100%;
        height: 100vh;
    }

    .error-message {
        font-weight: 500;
        font-size: 20px;
    }

    .error-working-directory {
        font-family: "SF Mono", monospace;
        font-weight: 300;
        font-size: 12px;
        margin-bottom: 12px;
    }

</style>
