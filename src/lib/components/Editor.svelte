
<div class="page">

    <div class="image-wrapper"
         on:mousemove={(e) => {mouseX = e.clientX; mouseY = e.clientY}}
         on:mouseup={() => mouseUpCounter += 1}
         on:wheel|preventDefault={handleImageScroll}
    >
        <div class="image-container" bind:this={imageContainer}>
            <img class="image" bind:this={image} src="" alt=""> <!-- Todo: add placeholder image -->
            {#if imageContainerDimension && imageContainerDimension[0] > 0}
                {#each metadata?.entities || [] as entity, index}
                    <BoundingBox position="{entity.boundingBox}"
                                 imageDimension="{imageDimension}"
                                 realDimension="{imageContainerDimension}"
                                 mousePosition="{[mouseX, mouseY]}"
                                 mouseUpCounter="{mouseUpCounter}"
                                 entity="{entity}"
                                 on:save={(e) => handleEntitiesUpdate(e, index)}
                    />
                {/each}
            {/if}
            </div>
    </div>

    <div class="controls-wrapper" class:disable-controls={disableUI}>

        <h1>{imagePath.split('/').pop()?.split('.')[0] || 'Editor'}</h1>

        <div class="control-block">
            <h2>Texterkennung</h2>
            {#each metadata?.entities || [] as entity, index}
                <Entity entity="{entity}"
                        on:save={(e) => handleEntitiesUpdate(e, index)}
                        on:delete={(_) => handleEntityDelete(index)}
                />
            {/each}
            <button on:click={addEntity} class="add-entity-button">Text hinzufügen</button>
        </div>

        <div class="control-block">
            <h2>Bildbearbeitung</h2>
            <BrightnessSlider
                    on:update={debounce(handleGradingUpdate)}
                    min="{0}" max="{2}" step="{0.01}" initial="{metadata?.grading?.brightness || 1}"
            />
            <ContrastSlider
                    on:update={debounce(handleGradingUpdate)}
                    min="{0}" max="{2}" step="{0.01}" initial="{metadata?.grading?.contrast || 1}"
            />
            <WhiteBalanceSlider
                on:update={debounce(handleGradingUpdate)}
                min="{0}" max="{2}" step="{0.01}" initial="{metadata?.grading?.whiteBalance || 1}"
            />
        </div>

        {#if dev}
        <div class="control-block">
            <h2>Debug</h2>
            <p>updateRequested: {updateRequested}</p>
            <p>isUpdating: {isUpdating}</p>
            <p>disableUI: {disableUI}</p>
        </div>
        {/if}

        <a href="/">Back</a>

    </div>

</div>



<script lang="ts">

    import {onMount} from 'svelte'
    import {invoke} from '@tauri-apps/api/tauri'
    import BrightnessSlider from '../../routes/editor/controls/BrightnessSlider.svelte'
    import ContrastSlider from '../../routes/editor/controls/ContrastSlider.svelte'
    import WhiteBalanceSlider from '../../routes/editor/controls/WhiteBalanceSlider.svelte'
    import Entity from '../../routes/editor/controls/Entity.svelte'
    import BoundingBox from '../../routes/editor/controls/BoundingBox.svelte'
    import {dev} from '$app/environment'

    export let imagePath: string
    let metadata: Metadata

    // state
    let disableUI = false
    let updateRequested = false
    let isUpdating = false

    let image: HTMLImageElement

    let mouseX = 0
    let mouseY = 0
    let mouseUpCounter = 0
    let imageContainer: HTMLDivElement
    let imageContainerDimension: [number, number]
    let imageDimension: [number, number]

    onMount(async () => {
        await getImage(imagePath)
        // fix size of image container
        imageContainer.style.width = `${imageContainerDimension[0]}px`
        imageContainer.style.minWidth = `${imageContainerDimension[0]}px`
    })

    async function getImage(path: string) {
        disableUI = true
        try {
            const result: EditorResponse = await invoke('get_image', {path: path})
            image.src = 'data:image/png;base64, ' + result[0]
            metadata = result[1]

            imageDimension = [image.naturalWidth, image.naturalHeight]
            while (imageDimension[0] == 0) {
                // wait until image is rendered to the dom
                await new Promise(r => setTimeout(r, 10))
                imageDimension = [image.naturalWidth, image.naturalHeight]
            }
            imageContainerDimension = [imageContainer.getBoundingClientRect().width, imageContainer.getBoundingClientRect().height]

            disableUI = false
        } catch (err) {
            console.error(err)
        }
    }

    function handleGradingUpdate(event: CustomEvent) {
        const eventType: string[] = event.detail.type
        const eventKey = eventType.pop() as string
        let eventTarget = metadata
        for (const key of eventType) {
            // @ts-ignore
            eventTarget = eventTarget[key]
        }
        // @ts-ignore
        eventTarget[eventKey] = event.detail.value
        if (isUpdating) {
            updateRequested = true
            return
        }
        metadata.grading.manuallyChanged = true
        updateImage()
    }

    async function updateImage() {
        try {
            isUpdating = true
            updateRequested = false

            const result = await invoke('update_image', {metadata: metadata})
            image.src = 'data:image/png;base64, ' + result

            isUpdating = false
            if (updateRequested) {
                await updateImage()
            }
        } catch (err) {
            console.error(err)
        }
    }

    function debounce(func: Function, delay = 50) {
        let timeoutId: ReturnType<typeof setTimeout>;

        return function(this: any, ...args: any[]) {
            clearTimeout(timeoutId);
            timeoutId = setTimeout(() => func.apply(this, args), delay);
        };
    }

    async function handleEntitiesUpdate(e: CustomEvent, index: number) {
        metadata.entities[index] = e.detail
        metadata = metadata
        let _ = await invoke('update_entities', {path: imagePath, metadata: metadata})
    }

    async function handleEntityDelete(index: number) {
        console.log('deleting entity...')
        metadata.entities.splice(index, 1)
        metadata = metadata
        let _ = await invoke('update_entities', {path: imagePath, metadata: metadata})
    }

    async function addEntity() {
        let x = imageDimension [0] / 2 - 600
        let y = imageDimension[1] / 2 - 200
        metadata.entities.push({
            label: 'client',
            text: '',
            boundingBox: {top: y, left: x, bottom: y + 100, right: x + 300},
            manuallyChanged: true,
        })
        metadata = metadata
        let _ = await invoke('update_entities', {path: imagePath, metadata: metadata})
    }

    /********** IMAGE NAVIGATION **********/

    let scale = 1
    let moveX = 0
    let moveY = 0

    $: matrix = [scale, 0, 0, scale, moveX, moveY]

    function handleImageScroll(e: WheelEvent) {
        scale = Math.max(0.5, Math.min(5, scale + e.deltaY * -0.005))
        imageContainer.style.transform = `matrix(${matrix})`
    }

</script>


<style>

    .page {
        display: flex;
    }

    .image-wrapper {
        width: 100%;
        height: 100vh;
        overflow: hidden;
        display: flex;
        justify-content: center;
        align-items: center;
        position: sticky;
        top: 0;
        user-select: none;
        -webkit-user-select: none;
    }

    .image-container {
        position: relative;
    }

    .image {
        pointer-events: none;
    }

    .controls-wrapper {
        min-width: 360px;
        padding: 12px;
        background-color: var(--background-color)
    }

    .control-block {
        display: flex;
        flex-direction: column;
        gap: 12px;

        background-color: var(--raised-color);
        padding: 20px;
        margin: 12px 0;
        border-radius: 8px;
    }

    .disable-controls {
        opacity: 0.5;
        pointer-events: none;
    }

    h1 {
        font-size: 28px;
        line-height: 28px;
    }

    h2 {
        font-size: 18px;
    }

    .add-entity-button {
        border: none;
        border-radius: 4px;
        padding: 4px 0;
        background-color: var(--background-color);
        font-weight: 500;
        cursor: pointer;
    }

</style>
