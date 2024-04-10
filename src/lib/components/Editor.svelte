<div class="wrapper unselectable">

    <div class="image-container">
        <div class="image-wrapper unselectable" bind:this={imageWrapper}
             on:mousemove={(e) => {mouseX = e.clientX; mouseY = e.clientY}}
             on:mouseup={() => mouseUpCounter += 1}
             on:wheel|preventDefault={handleImageScroll}
        >
            <img bind:this={image} src="" alt="" class="unselectable"> <!-- Todo: add placeholder image -->
            {#each metadata?.entities || [] as entity, index}
                <BoundingBox position="{entity.boundingBox}"
                             imageDimension="{imageDimension}"
                             realDimension="{[image.getBoundingClientRect().width, image.getBoundingClientRect().height]}"
                             mousePosition="{[mouseX, mouseY]}"
                             mouseUpCounter="{mouseUpCounter}"
                             entity="{entity}"
                             translationMatrix="{inverseMatrix}"
                             on:save={(e) => handleEntitiesUpdate(e, index)}
                />
            {/each}
        </div>
    </div>

    <div class="controls-wrapper" class:disable-controls={disableUI}>

        <h1>{imagePath.split('/').pop()?.split('.')[0] || 'Editor'}</h1>

        <div class="control-block">
            <h2>Texterkennung</h2>
            {#each metadata?.entities || [] as entity}
                <Entity entity="{entity}" />
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

        <div class="control-block">
            <h2>Debug</h2>
            <p>updateRequested: {updateRequested}</p>
            <p>isUpdating: {isUpdating}</p>
            <p>disableUI: {disableUI}</p>
        </div>

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
    $: imageDimension = [image?.naturalWidth, image?.naturalWidth] as [Optional<number>, Optional<number>]

    onMount(() => {
        getImage(imagePath)
    })

    async function getImage(path: string) {
        disableUI = true
        try {
            const result: EditorResponse = await invoke('get_image', {path: path})
            image.src = 'data:image/png;base64, ' + result[0]
            metadata = result[1]
            console.log('metadata:', metadata)
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
            eventTarget = eventTarget[key]
        }
        eventTarget[eventKey] = event.detail.value
        if (isUpdating) {
            updateRequested = true
            return
        }
        updateImage()
        console.log('handleUpdate done')
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
        let res = await invoke('update_entities', {path: imagePath, metadata: metadata})
    }

    function addEntity() {
        // call rust function
    }

    /********** IMAGE NAVIGATION **********/

    let imageWrapper: HTMLDivElement
    let scale = 1
    let moveX = 0
    let moveY = 0

    $: matrix = [scale, 0, 0, scale, moveX, moveY]
    $: inverseMatrix = [
        1 / scale, 0, 0,
        0, 1 / scale, 0,
        - moveX / scale, - moveY / scale, 1
    ]

    function handleImageScroll(e: WheelEvent) {
        scale = Math.max(0.5, Math.min(5, scale + e.deltaY * -0.005))
        /*if (0.9 < scale && scale < 1.1) {  // snap to 100%
            let localMatrix = matrix
            localMatrix[0] = 1
            localMatrix[3] = 1
            imageWrapper.style.transform = `matrix(${matrix})`
            return
        }*/
        imageWrapper.style.transform = `matrix(${matrix})`
    }

</script>


<style>

    .wrapper {
        position: relative;
        display: flex;
        align-items: center;
    }

    .image-container {
        overflow: hidden;
        width: 100%;
        height: 100vh;
        position: relative;
    }

    .image-wrapper {
        position: relative;
        /*transform-origin: top left;*/
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

    .unselectable {
        user-select: none;
        --webkit-user-select: none;
    }

</style>
