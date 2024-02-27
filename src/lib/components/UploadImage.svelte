<button on:click={handleHelloClick}>Hello from Rust</button>
<button on:click={loadImage}>Load image</button>

<button on:click={uploadImage}>Upload image</button>

<script lang="ts">
    import { invoke } from '@tauri-apps/api/tauri'

    // const invoke = window.__TAURI__.invoke
    function handleHelloClick() {
        invoke('my_custom_command', {msg: 'Joel'})
    }

    async function loadImage() {
        const result = await invoke('load_image', {path: '/Users/kl/Kevin/Projects/ASLA/temp images/cre_6087_11.jpg'})
        console.log('result:', result)
    }

    async function uploadImage() {
        try {
            const result = await invoke('upload_image', {
                path: '/Users/kl/Kevin/Projects/ASLA/temp images/cre_6087_11.jpg',
                name: 'test_image'
            })
            console.log('result:', result)
        } catch (err) {
            console.error(err)
        }
    }

</script>
