<button on:click={async () => { selectedDirectory = await selectDirectory() || selectedDirectory }}>Open directory</button>
<p>Selected directory: {selectedDirectory || '-'}</p>

<button on:click={postImage}>Fetch</button>

<button on:click={getImages}>Get images</button>
{#if selectedDirectory}
    <p>First selected image:</p>
{/if}

<script lang="ts">
    import { open } from '@tauri-apps/api/dialog'
    import {fs} from '@tauri-apps/api'
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
        const response = await fetch('https://jsonplaceholder.typicode.com/posts/1', {
            method: 'GET'
        })
        console.log('response:', response)
    }

    async function getImages() {
        if (typeof selectedDirectory !== 'string' || ! await fs.exists(selectedDirectory)) {
            console.warn('trying to load images from invalid directory path')
            return
        }

        const files = await fs.readDir(selectedDirectory)
        console.log('FILES:')
        for (const file of files) {
            console.log(file)
        }
    }

</script>
