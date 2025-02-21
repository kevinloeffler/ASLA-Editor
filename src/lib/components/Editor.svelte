<svelte:window on:keydown={handleKeyDown} />

<div class="page">

    {#if showErrorMsg}
        <div class="error-msg">
            <div>
                <h2>Fehler beim Laden des Bildes</h2>
                <button on:click={() => dispatch('handle_image_load_error')}>Anderes Bild öffnen</button>
            </div>
        </div>
    {/if}

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

           {#each displayEntities.filter(e => e.entity.hasBoundingBox) || [] as entity (entity)}
                <BoundingBox displayEntity="{entity}" scaleFactor="{scaleTranslationFactor}"
                             on:save={(_) => handleMetadataUpdate()}
                />
            {/each}

            <DrawBoundingBox scalingFactor="{scaleTranslationFactor}" bind:this={drawingOverlay} />

            <FormatOverlay metadata={metadata} resizing={isResizing} on:resize={(e) => metadata = e.detail}
                           scaleTranslationFactor={scaleTranslationFactor} scale={scale}
            />
        </div>

    </div>

    <EntitySelection position={entitySelectionPosition} entity={entitySelectionEntity} drawUpdate={handleMetadataUpdate} />

    <!-- CONTROLS -->

    <div class="side-panel">
        <div class="controls-wrapper" class:disable-controls={disableUI}>

            <h1>{imagePath.split('/').pop()?.split('.')[0] || 'Editor'}</h1>

            <div class="control-block">
                <h2>Texterkennung</h2>
                {#each displayEntities || [] as displayEntity (displayEntity.entity)}
                    <EntityComponent bind:displayEntity="{displayEntity}"
                                     on:save={(e) => handleMetadataUpdate()}
                                     on:delete={(_) => handleEntityDelete(displayEntity)}
                                     on:start-drawing={(e) => startDrawingBoundingBox(e.detail, false)}
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

            <div class="control-block">
                <FormatControls on:toggle={() => isResizing = !isResizing}/>
            </div>

            {#if dev}
                <div class="control-block">
                    <h2>Debug</h2>
                    <p>updateRequested: {updateRequested}</p>
                    <p>isUpdating: {isUpdating}</p>
                    <p>disableUI: {disableUI}</p>
                </div>
            {/if}

        </div>

        <div class="navigation">
            <button on:click={() => dispatch('previous_image')}>
                <img src="/assets/icons/chevron-left.svg" alt="Bild Zurück">
            </button>
            <button on:click={() => location.href = '/projects/' + project?.code}>Projekt</button>
            <button on:click={() => location.href = '/'}>Startseite</button>
            <button on:click={() => dispatch('next_image')}>
                <img src="/assets/icons/chevron-right.svg" alt="Bild Vorwärts">
            </button>
        </div>

    </div>

    <!-- TOAST -->

    <Toast bind:this={toast}/>

</div>



<script lang="ts">

    import {createEventDispatcher} from 'svelte'
    import {invoke} from '@tauri-apps/api/tauri'
    import {dev} from '$app/environment'
    import BrightnessSlider from '../../routes/editor/controls/BrightnessSlider.svelte'
    import ContrastSlider from '../../routes/editor/controls/ContrastSlider.svelte'
    import WhiteBalanceSlider from '../../routes/editor/controls/WhiteBalanceSlider.svelte'
    import EntityComponent from '../../routes/editor/controls/Entity.svelte'
    import BoundingBox from '../../routes/editor/BoundingBox.svelte'
    import Toast from '../../routes/editor/Toast.svelte'
    import DrawBoundingBox from '../../routes/editor/DrawBoundingBox.svelte'
    import {hashCode} from '$lib/services/entity-service'
    import EntitySelection from '../../routes/editor/EntitySelection.svelte'
    import FormatOverlay from '../../routes/editor/FormatOverlay.svelte'
    import FormatControls from '../../routes/editor/controls/FormatControls.svelte'

    const dispatch = createEventDispatcher()

    export let imagePath: string
    export let project: Optional<Project>

    let metadata: Metadata
    let displayEntities: DisplayEntity[] = []
    $: metadata, handleMetadataUpdate()
    $: refreshDisplayEntities(metadata?.entities)

    let toast: Toast

    let image: HTMLImageElement
    let imageContainer: HTMLDivElement

    $: getImage(imagePath)  // reload image if path changes

    let imageWidth: number = 0
    let imageHeight: number = 0
    let containerWidth: number
    $: scaleTranslationFactor = imageWidth / containerWidth

    let disableUI = false
    let isUpdating = false
    let updateRequested = false

    let showErrorMsg = false

    let drawingOverlay: DrawBoundingBox
    let entitySelectionEntity: Entity
    let entitySelectionPosition: Optional<{x: number; y: number}>

    let isResizing: boolean = false

    function refreshDisplayEntities(entities: Optional<Entity[]>) {
        if (!entities) return []
        displayEntities = entities.map(e => ({entity: e, highlight: false})) || [] as DisplayEntity[]
    }

    async function startDrawingBoundingBox(displayEntity: DisplayEntity, isNew: boolean) {
        toast.show('Box von der oberen linken Ecke aus aufziehen', ()=>{console.log('hello')}, undefined)
        const { newEntity, mouse } = await drawingOverlay.draw(displayEntity.entity)
        toast.hide()

        if (isNew) {
            metadata.entities.push(newEntity)
            await handleMetadataUpdate()

            // Set entity selection overlay position
            let xPosition = mouse.x
            let yPosition = mouse.y

            const sideBar = document.querySelector('.side-panel')! as HTMLElement
            const rightEditorBorder = sideBar.getBoundingClientRect().x
            const bottomEditorBorder = sideBar.getBoundingClientRect().height

            if (mouse.x + 190 > rightEditorBorder) {
                xPosition = xPosition - 140
            }
            if (mouse.y + 250 > bottomEditorBorder) {
                yPosition = yPosition - 200
            }

            entitySelectionPosition = {x: xPosition, y: yPosition}
            entitySelectionEntity = newEntity
        }
        await handleMetadataUpdate()
    }

    /********** DATA **********/

    async function getImage(path: string) {
        showErrorMsg = false
        disableUI = true
        try {
            const response: EditorResponse = await invoke('get_image', {path: path})

            const imageLoaded = new Promise(resolve => {
                image.onload = resolve
                image.src = 'data:image/png;base64, ' + response[0]
            })

            await imageLoaded

            imageWidth = image.naturalWidth
            imageHeight = image.naturalHeight
            metadata = response[1]
            disableUI = false
        } catch (err: any) {
            if (err.message === 'No such file or directory (os error 2)') {
                showErrorMsg = true
            } else {
                // handle other errors...
                console.error(err)
            }
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

    async function handleMetadataUpdate() {
        if (!metadata) return
        metadata = metadata
        let _ = await invoke('update_metadata', {path: imagePath, metadata: metadata})
    }

    async function handleEntityDelete(displayEntity: DisplayEntity) {
        const index = metadata.entities.findIndex(el => hashCode(el) === hashCode(displayEntity.entity))
        if (index === -1) return
        metadata.entities.splice(index, 1)
        metadata = metadata
        let _ = await invoke('update_metadata', {path: imagePath, metadata: metadata})
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
        const newDisplayEntity: DisplayEntity = {
            entity: newEntity,
            highlight: false,
        }
        await startDrawingBoundingBox(newDisplayEntity, true)

        let elements: NodeListOf<HTMLInputElement> = document.querySelectorAll('.entity-text-input')
        if (elements.length > 0) {
            elements[elements.length - 1].focus()
        }
    }

    /********* KEYBOARD SHORTCUTS *********/

    async function handleKeyDown(event: KeyboardEvent) {
        if (!event.ctrlKey && !event.metaKey) return

        event.preventDefault()
        switch (event.key) {
            case '0':
                resetTransformation()
                break
            case 'n':
                await addEntity()
                break
            case 'ArrowRight':
                dispatch('next_image')
                break
            case 'ArrowLeft':
                dispatch('previous_image')
                break
        }
    }

</script>

<style>

    .page {
        display: flex;
    }

    .error-msg {
        position: fixed;
        top: 0;
        bottom: 0;
        left: 0;
        right: 0;
        display: flex;
        align-items: center;
        background-color: rgba(255, 255, 255, 0.5);
        padding-left: 80px;
        z-index: 999;
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

    .side-panel {
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

    .navigation {
        display: flex;
        justify-content: space-between;
        gap: 4px;

        position: sticky;
        bottom: 0;
        padding-bottom: 8px;
        padding-top: 33px;
        background: linear-gradient(to top, var(--background-color) 66%, rgba(255, 255, 255, 0) 100%);
        z-index: 999;
    }

    .navigation > button {
        display: flex;
        justify-content: center;
        align-items: center;
        color: var(--background-color);
        background-color: var(--text-color);
        border: none;
        border-radius: 8px;
        width: 100%;
        padding: 6px 20px;
        cursor: pointer;
        transition-duration: 100ms;
    }

    .navigation > button:hover {
        opacity: 0.8;
    }


</style>