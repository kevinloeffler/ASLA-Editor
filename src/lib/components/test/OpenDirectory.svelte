<button on:click={async () => { selectedDirectory = await selectDirectory() || selectedDirectory }}>Open directory</button>
<p>Selected directory: {selectedDirectory || '-'}</p>

<button on:click={postImage}>Fetch</button>

<button on:click={getImages}>Get images</button>
{#if selectedDirectory}
    <p>First selected image:</p>
{/if}

<script lang="ts">
    import { open } from '@tauri-apps/api/dialog'
    import { fs } from '@tauri-apps/api'
    import { fetch } from '@tauri-apps/api/http'

    let selectedDirectory: Optional<string>

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

    async function postImage() {
        const images = await getImages()
        const formData = new FormData()

        formData.append('name', 'test')
        formData.append('image', images[0]);

        const options = {
            method: 'POST',
            body: formData,
        }

        const response = await fetch('http://localhost:8000/image/',{
            method: 'POST', body: { type: 'Form', payload: formData }
        })
    }

    async function getImages() {
        if (typeof selectedDirectory !== 'string' || ! await fs.exists(selectedDirectory)) {
            console.warn('trying to load images from invalid directory path')
            return
        }

        return await fs.readDir(selectedDirectory)
    }

</script>
