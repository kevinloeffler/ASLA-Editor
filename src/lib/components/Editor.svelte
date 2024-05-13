<div class="page">

    <!-- IMAGE -->

    <div class="image-wrapper"
         on:mousemove={handleMouseMove}
         on:mousedown={handleMouseDown}
         on:mouseup={handleMouseUp}
         on:wheel|preventDefault={handleImageScroll}
         on:mouseleave={() => dragging = false}
         class:dragging={dragging}
         aria-hidden="true"
    >
        <div class="image-container" bind:this={imageContainer}
             bind:clientWidth={containerWidth}
        >
            <img src="" alt="" bind:this={image} class="image">

           {#each displayEntities.filter(e => e.entity.hasBoundingBox) || [] as entity, index}
                <BoundingBox displayEntity="{entity}" scaleFactor="{scaleTranslationFactor}"
                             on:save={(e) => handleEntitiesUpdate(e, index)}
                />
            {/each}

            <DrawBoundingBox scalingFactor="{scaleTranslationFactor}" bind:this={drawingOverlay} />
        </div>

    </div>

    <!-- CONTROLS -->

    <div class="controls-wrapper" class:disable-controls={disableUI}>

        <h1>{imagePath.split('/').pop()?.split('.')[0] || 'Editor'}</h1>

        <div class="control-block">
            <h2>Texterkennung</h2>
            {#each displayEntities || [] as displayEntity, index}
                <EntityComponent bind:displayEntity="{displayEntity}"
                                 on:save={(e) => handleEntitiesUpdate(e, index)}
                                 on:delete={(_) => handleEntityDelete(index)}
                                 on:start-drawing={(e) => startDrawingBoundingBox(e, index)}
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

        <a href="/">Home</a>
        <button on:click={resetTransformation}>Center image</button>  <!-- TODO: implement this with a CMD + 0 -->

    </div>

    <!-- TOAST -->

    <Toast bind:this={toast}/>

</div>



<script lang="ts">

    import {onMount} from 'svelte'
    import {invoke} from '@tauri-apps/api/tauri'
    import {dev} from '$app/environment'
    import BrightnessSlider from '../../routes/editor/controls/BrightnessSlider.svelte'
    import ContrastSlider from '../../routes/editor/controls/ContrastSlider.svelte'
    import WhiteBalanceSlider from '../../routes/editor/controls/WhiteBalanceSlider.svelte'
    import EntityComponent from '../../routes/editor/controls/Entity.svelte'
    import BoundingBox from '../../routes/editor/BoundingBox.svelte'
    import Toast from '../../routes/editor/Toast.svelte'
    import DrawBoundingBox from '../../routes/editor/DrawBoundingBox.svelte'

    export let imagePath: string
    let metadata: Metadata
    let displayEntities: DisplayEntity[] = []
    $: refreshDisplayEntities(metadata?.entities)

    let toast: Toast

    let image: HTMLImageElement // new Image()
    let imageContainer: HTMLDivElement

    let imageWidth: number = 0
    let imageHeight: number = 0
    let containerWidth: number
    $: scaleTranslationFactor = imageWidth / containerWidth

    let disableUI = false
    let isUpdating = false
    let updateRequested = false

    let drawingOverlay: DrawBoundingBox

    onMount( async () => {
        await getImage(imagePath)
    })

    function refreshDisplayEntities(entities: Optional<Entity[]>) {
        if (!entities) return []
        displayEntities = entities.map(e => ({entity: e, highlight: false})) || [] as DisplayEntity[]
    }

    async function startDrawingBoundingBox(e: CustomEvent, index: number) {
        toast.show('Box von der oberen linken Ecke aus aufziehen', ()=>{console.log('hello')}, undefined)
        const newEntity = await drawingOverlay.draw(e.detail.entity)
        toast.hide()

        await handleEntitiesUpdate({detail: newEntity} as CustomEvent, index)
    }

    /********** DATA **********/

    async function getImage(path: string) {
        disableUI = true
        try {
            const response: EditorResponse = await invoke('get_image', {path: path})

            image.src = 'data:image/png;base64, ' + response[0]
            imageWidth = image.naturalWidth
            imageHeight = image.naturalHeight
            metadata = response[1]
            disableUI = false
        } catch (err) {
            // TODO: handle image load error
            console.error(err)
        }
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

    function debounce(func: Function, delay = 50) {
        let timeoutId: ReturnType<typeof setTimeout>;
        return function(this: any, ...args: any[]) {
            clearTimeout(timeoutId);
            timeoutId = setTimeout(() => func.apply(this, args), delay);
        };
    }

    /********** NAVIGATION **********/

    let scale = 1
    let moveX = 0
    let moveY = 0

    let dragging = false

    let lastMouseX = 0
    let lastMouseY = 0

    function handleImageScroll(e: WheelEvent) {
        const newScale = Math.max(0.5, Math.min(5, scale + e.deltaY * -0.005))

        // apply translation according to cursor position -> scroll center is the mouse
        // the last mouse position is saved, if the scrolls when the mouse is not on the image wrapper or container
        //  (e.g. a bounding box) then the cached value is used.
        let x = lastMouseX
        let y = lastMouseY

        if (e.target === e.currentTarget || e.target === imageContainer) {  // check if the mouse is on the image
            x = lastMouseX = Math.round((e.offsetX * scaleTranslationFactor) - imageWidth / 2)
            y = lastMouseY = Math.round((e.offsetY * scaleTranslationFactor) - imageHeight / 2)
        }

        const x1 = scale * x + moveX
        const y1 = scale * y + moveY
        const x2 = newScale * x + moveX
        const y2 = newScale * y + moveY
        const deltaX = Math.round(x2 - x1)
        const deltaY = Math.round(y2 - y1)

        moveX -= deltaX * (1 / scaleTranslationFactor)
        moveY -= deltaY * (1 / scaleTranslationFactor)

        const margin = Math.floor(containerWidth * 0.05) - newScale * 200
        moveX = Math.max(-containerWidth / 2 + margin, Math.min(containerWidth / 2 - margin, moveX))
        moveY = Math.max(-containerWidth / 2 + margin, Math.min(containerWidth / 2 - margin, moveY))

        scale = newScale
        render()
    }

    function handleMouseMove(e: MouseEvent) {
        if (dragging) {
            const margin = Math.floor(containerWidth * 0.05) - scale * 200
            moveX = Math.max(-containerWidth / 2 + margin, Math.min(containerWidth / 2 - margin, moveX + e.movementX))
            moveY = Math.max(-containerWidth / 2 + margin, Math.min(containerWidth / 2 - margin, moveY + e.movementY))
            render()
        }
    }

    function handleMouseDown() { dragging = true }
    function handleMouseUp() { dragging = false }

    function resetTransformation() {
        scale = 1
        moveX = 0
        moveY = 0
        render()
    }

    function render() {
        const matrix = [scale, 0, 0, scale, moveX, moveY]
        imageContainer.style.transform = `matrix(${matrix})`
    }

    /********* ENTITIES *********/

    async function handleEntitiesUpdate(e: CustomEvent, index: number) {
        metadata.entities[index] = e.detail
        metadata = metadata
        let _ = await invoke('update_entities', {path: imagePath, metadata: metadata})
    }

    async function handleEntityDelete(index: number) {
        metadata.entities.splice(index, 1)
        metadata = metadata
        let _ = await invoke('update_entities', {path: imagePath, metadata: metadata})
    }

    async function addEntity() {
        let x = imageWidth / 2 - 600
        let y = imageHeight / 2 - 200
        const newEntity = {
            label: 'O',
            text: '',
            hasBoundingBox: false,
            boundingBox: {top: y, left: x, bottom: y + 100, right: x + 300},
            manuallyChanged: true,
        }
        // metadata.entities.push(newEntity)
        await startDrawingBoundingBox({detail: {entity: newEntity}} as CustomEvent, metadata.entities.length)
        // await handleEntitiesUpdate({detail: newEntity} as CustomEvent, metadata.entities.length)
        // let _ = await invoke('update_entities', {path: imagePath, metadata: metadata})
    }


</script>

<style>

    .page {
        display: flex;
    }

    .image-wrapper {
        width: 100%;
        height: 100vh;

        position: sticky;
        top: 0;

        display: flex;
        justify-content: center;
        align-items: center;

        overflow: hidden;
    }

    .image-container {
        position: relative;
        transform-origin: center;

        user-select: none;
        -webkit-user-select: none;
    }

    .image {
        pointer-events: none;
    }

    .dragging {
        cursor: grab;
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

    .add-entity-button {
        border: none;
        border-radius: 4px;
        padding: 4px 0;
        background-color: var(--background-color);
        font-weight: 500;
        cursor: pointer;
    }

    h1 {
        font-size: 28px;
        line-height: 28px;
    }

    h2 {
        font-size: 18px;
    }


</style>