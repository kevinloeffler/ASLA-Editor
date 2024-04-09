
<button on:click={async () => { selectedDirectory = await selectDirectory() || selectedDirectory }}>Open directory</button>
<p>Selected directory: {selectedDirectory || '-'}</p>

{#if selectedDirectory}
    {#if !isUploading}
        <button on:click={startUpload}>Start upload</button>
    {:else}
        <button on:click={() => isUploading = false}>Stop upload</button>
    {/if}
{/if}

<script lang="ts">
    import {open} from '@tauri-apps/api/dialog'
    import {fs} from '@tauri-apps/api'
    import type {FileEntry} from '@tauri-apps/api/fs'
    import {ImageDirectoryManager} from '$lib/services/image-directory-manager.js'

    let selectedDirectory: Optional<string>

    const directoryManager = new ImageDirectoryManager()
    let isUploading = false
    const directoryUpdateInterval = 3000

    $: console.log('isUploading:', isUploading)

    async function selectDirectory(): Promise<Optional<string>> {
        let selected = await open({
            directory: true,
            multiple: false,
            defaultPath: selectedDirectory || '/',
        })

        if (selected === null) {
            console.warn('No directory selected')
            return undefined
        } else if (Array.isArray(selected)) { selected = selected[0] }

        return selected
    }

    async function getImages(): Promise<FileEntry[]> {
        if (typeof selectedDirectory !== 'string' || ! await fs.exists(selectedDirectory)) {
            console.error('trying to load images from invalid directory path')
            return []
        }

        const files = await fs.readDir(selectedDirectory)
        console.log('files:', files)
        const images = files.filter(file => isImage(file.name))
        console.log('images:', images)
        return images
    }

    function isImage(filename: Optional<string>): boolean {
        const imageSuffixes = ['.jpg', '.jpeg', '.png', '.tiff']
        for (let suffix of imageSuffixes) {
            if (filename?.toLowerCase().endsWith(suffix)) {
                return true
            }
        }
        return false
    }

    async function startUpload() {
        console.log('starting upload...')
        isUploading = true
        await directoryManager.startUpload()
        const upload = async () => {
            if (!isUploading) {
                directoryManager.stopUpload()
                clearInterval(interval)
            }
            await directoryManager.update(await getImages())
        }
        const interval = setInterval(upload, directoryUpdateInterval)
    }




</script>
