
<div>

    <div class="header">
        <h3>Upload</h3>
        {#if isUploading}
            <button class="upload-button upload-button-stop" on:click={() => UPLOAD_SERVICE.stopUpload()}>
                Upload stoppen
            </button>
        {:else}
            <button class="upload-button" on:click={async () => await UPLOAD_SERVICE.startUpload()}>
                Upload starten
            </button>
        {/if}
    </div>

    <div class="labels">
        <p>Bild</p>
        <p>Projekt</p>
        <p>Status</p>
    </div>

    {#each images as image}
        <div class="upload-element">
            <p class="upload-name">{image.name}</p>
            <p class="upload-project">PRJ</p>
            <div class="upload-icon">
                {#if failedImages.includes(image.name)}
                    <button on:click={() => UPLOAD_SERVICE.whitelistImage(image.name)} class="upload-retry-button">
                        <img width="24" src="/assets/icons/upload-retry.svg" alt="Wiederholen" class="upload-retry">
                        <img width="24" src="/assets/icons/upload-failed.svg" alt="Fehler" class="upload-failed">
                    </button>
                {:else if image.name === currentUpload}
                    <img width="24" src="/assets/icons/upload-working.svg" alt="Upload">
                {:else}
                    <img width="24" src="/assets/icons/upload-queue.svg" alt="Warteschlange">
                {/if}
            </div>
        </div>
    {/each}

</div>


<script lang="ts">

    import {UPLOAD_SERVICE} from '$lib/services/upload-service'
    import {onDestroy, onMount} from 'svelte'
    UPLOAD_SERVICE.setDirectory('/Users/kl/Kevin/Projects/ASLA/ASLA Editor test dir/upload')

    let isUploading: boolean
    UPLOAD_SERVICE.uploading.subscribe(value => isUploading = value)

    let images: any[]
    UPLOAD_SERVICE.images.subscribe(value => images = value)

    let failedImages: string[]
    UPLOAD_SERVICE.failedImages.subscribe(value => failedImages = value)

    let currentUpload: Optional<string>
    UPLOAD_SERVICE.currentUpload.subscribe(value => currentUpload = value)

    let timer: number
    onMount(() => {
        timer = setInterval(() => UPLOAD_SERVICE.refresh(), 2000)
    })

    onDestroy(() => {
        UPLOAD_SERVICE.refresh()
        clearInterval(timer)
    })

</script>


<style>

    .header {
        display: flex;
        justify-content: space-between;
        align-items: baseline;
        margin-bottom: 12px;
    }

    .upload-button {
        font-weight: 500;
        padding: 4px 12px;
        color: var(--background-color);
        background-color: var(--green);
        border-radius: 8px;
        border: none;
    }

    .upload-button-stop {
        background-color: var(--red);
    }

    .labels {
        display: flex;
        padding: 0 8px;
    }

    .labels > p:first-child {
        width: 80%;
    }

    .labels > p:nth-child(2) {
        width: 90px;
    }

    .labels > p:nth-child(3) {
        width: 60px;
    }

    .upload-element {
        display: flex;
        align-items: center;
        padding: 4px 8px;
        margin-bottom: 4px;
        border-radius: 8px;
        background-color: var(--background-color);
    }

    .upload-name {
        width: 80%;
    }

    .upload-project {
        width: 90px;
    }

    .upload-icon {
        width: 60px;
        display: flex;
        align-items: center;
    }

    .upload-retry-button {
        background-color: transparent;
        border: none;
        padding: 0;
        margin: 0;
        cursor: pointer;
    }

    .upload-retry-button > .upload-retry {
        display: none;
    }

    .upload-retry-button:hover > .upload-retry {
        display: block;
    }

    .upload-retry-button:hover > .upload-failed {
        display: none;
    }

</style>
