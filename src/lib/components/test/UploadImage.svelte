<button on:click={uploadImage}>Upload image</button>

<script lang="ts">
    import { invoke } from '@tauri-apps/api/tauri'

    async function uploadImage() {
        try {
            const result = await invoke('upload_image', {
                path: '/Users/kl/Kevin/Projects/ASLA/temp images/cre_6087_11.jpg',
                name: 'test_image'
            })
        } catch (err) {
            console.error(err)
        }
    }

</script>
