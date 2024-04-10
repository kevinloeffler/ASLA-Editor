<div bind:this={box} class="bounding-box" class:focus={editMode}
     on:mouseenter={() => showEditButton = true}
     on:mouseleave={() => showEditButton = false}>

    {#if showEditButton || editMode}
        {#if editMode}
            <button on:click={() => save()}><img src="/assets/icons/editor-checkmark.svg" alt=""></button>
        {:else}
            <button on:click={() => editMode = true}><img src="/assets/icons/editor-crop-icon.svg" alt=""></button>
        {/if}
    {/if}

    {#if editMode}
        <div id="handle-top" class="handle" on:mousedown={() => dragStart('top')}></div>
        <div id="handle-right" class="handle" on:mousedown={() => dragStart('right')}></div>
        <div id="handle-bottom" class="handle" on:mousedown={() => dragStart('bottom')}></div>
        <div id="handle-left" class="handle" on:mousedown={() => dragStart('left')}></div>
    {/if}
</div>


<script lang="ts">

    import {createEventDispatcher, onMount} from 'svelte'

    const dispatch = createEventDispatcher()

    export let position: {top: number, right: number, bottom: number, left: number}
    export let imageDimension: [Optional<number>, Optional<number>]  // width, height
    export let realDimension: [Optional<number>, Optional<number>]  // width, height
    export let mousePosition: [number, number]  // x, y viewport coordinates
    export let mouseUpCounter: number
    export let entity: Entity
    export let translationMatrix: number[]

    let box: HTMLDivElement

    let showEditButton = false
    let editMode: boolean

    let conversionFactor = (realDimension[0] || 1) / (imageDimension[0] || 1)

    let width = Math.round((position.right - position.left) * conversionFactor)
    let height = Math.round((position.bottom - position.top) * conversionFactor)
    let top = Math.round(position.top * conversionFactor)
    let left = Math.round(position.left * conversionFactor)

    console.log('imageDim:', imageDimension)
    console.log('realDim:', realDimension)
    console.log('conv factor:', conversionFactor)
    console.log(`width: ${width}, height: ${height}`)
    console.log(`top: ${top}, left: ${left}`)

    onMount(() => {
        // let [newLeft, newTop] = translate(left, top)
        // let [newWidth, newHeight] = translate(left + width, top + height)
        box.style.top = `${top}px`
        box.style.left = `${left}px`
        box.style.width = `${width}px`
        box.style.height = `${height}px`
    })

    function translate(x: number, y: number) {
        let newX = (translationMatrix[0] * x) / (translationMatrix[2] * x + translationMatrix[5] * y + 1)
        let newY = (translationMatrix[4] * y) / (translationMatrix[2] * x + translationMatrix[5] * y + 1)
        return [newX, newY]
    }


    // dragging
    let isDragging = false
    let currentElement: 'top' | 'right' | 'bottom' | 'left'

    function dragStart(position: 'top' | 'right' | 'bottom' | 'left') {
        isDragging = true
        currentElement = position
    }

    function dragEnd() {
        isDragging = false
    }

    $: isDragging && update(mousePosition[0], mousePosition[1])
    $: mouseUpCounter, dragEnd()

    function update(x: number, y: number) {
        let [tx, ty] = translate(x, y)
        switch (currentElement) {
            case 'top':
                console.log('offsetTop:', box.offsetTop)
                console.log('BoundingClientRect.top:', box.getBoundingClientRect().top)
                const topPos = ty - box.offsetTop - 1
                box.style.height = `${box.offsetHeight - topPos}px`
                box.style.top = `${box.offsetTop + topPos}px`
                break
            case 'bottom':
                const bottomPos = ty - box.offsetTop - box.offsetHeight + 1
                box.style.height = `${box.offsetHeight + bottomPos}px`
                break
            case 'left':
                const leftPos = tx - box.offsetLeft - 1
                box.style.width = `${box.offsetWidth - leftPos}px`
                box.style.left = `${box.offsetLeft + leftPos}px`
                break
            case 'right':
                const rightPos = tx - box.offsetLeft - box.offsetWidth + 1
                box.style.width = `${box.offsetWidth + rightPos}px`
                break
        }
    }

    function save() {
        editMode = false
        let dimensions = box.getBoundingClientRect()
        let newBoundingBoxes = {
            top: Math.round(box.offsetTop / conversionFactor),
            right: Math.round((box.offsetLeft + box.offsetWidth) / conversionFactor),
            bottom: Math.round((box.offsetTop + box.offsetHeight) / conversionFactor),
            left: Math.round(box.offsetLeft / conversionFactor),
            // top: Math.round(dimensions.top / conversionFactor),
            // right: Math.round(dimensions.right / conversionFactor),
            // bottom: Math.round(dimensions.bottom / conversionFactor),
            // left: Math.round(dimensions.left / conversionFactor),
        }
        entity.boundingBox = newBoundingBoxes
        entity.manuallyChanged = true
        dispatch('save', entity)
    }

</script>


<style>

    .bounding-box {
        position: absolute;
        border: 2px var(--red) solid;
    }

    button {
        position: absolute;
        top: -20px;
        left: -2px;

        display: flex;
        align-items: center;
        justify-content: center;

        width: 28px;
        height: 20px;

        border-radius: 4px;
        border: none;

        background-color: var(--red);

        cursor: pointer;
    }

    .handle {
        position: absolute;
        background-color: var(--red);
        border-radius: 4px;
    }

    #handle-top {
        top: -5px;
        left: calc(50% - 8px);
        width: 20px;
        height: 8px;
        cursor: row-resize;
    }

    #handle-right {
        right: -5px;
        top: calc(50% - 10px);
        width: 8px;
        height: 20px;
        cursor: col-resize;
    }

    #handle-bottom {
        bottom: -5px;
        left: calc(50% - 8px);
        width: 20px;
        height: 8px;
        cursor: row-resize;
    }

    #handle-left {
        left: -5px;
        top: calc(50% - 10px);
        width: 8px;
        height: 20px;
        cursor: col-resize;
    }

    .focus {
        z-index: 99;
    }

</style>
