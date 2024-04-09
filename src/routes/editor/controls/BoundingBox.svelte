<div bind:this={box} class="bounding-box"
     on:mouseenter={() => showEditButton = true}
     on:mouseleave={() => showEditButton = false}>

    <div class="rectangle"></div>

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
        box.style.top = `${top - 10}px`
        box.style.left = `${left - 10}px`
        box.style.width = `${width + 20}px`
        box.style.height = `${height + 20}px`
    })

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
        switch (currentElement) {
            case 'top':
                const topPos = y - box.offsetTop - 9
                box.style.height = `${box.offsetHeight - topPos}px`
                box.style.top = `${box.offsetTop + topPos}px`
                break
            case 'bottom':
                const bottomPos = y - box.offsetTop - box.offsetHeight + 10
                box.style.height = `${box.offsetHeight + bottomPos}px`
                break
            case 'left':
                const leftPos = x - box.offsetLeft - 9
                box.style.width = `${box.offsetWidth - leftPos}px`
                box.style.left = `${box.offsetLeft + leftPos}px`
                break
            case 'right':
                const rightPos = x - box.offsetLeft - box.offsetWidth + 10
                box.style.width = `${box.offsetWidth + rightPos}px`
                break
        }
    }

    function save() {
        editMode = false
        let dimensions = box.getBoundingClientRect()
        let newBoundingBoxes = {
            top: Math.round(dimensions.top / conversionFactor),
            right: Math.round(dimensions.right / conversionFactor),
            bottom: Math.round(dimensions.bottom / conversionFactor),
            left: Math.round(dimensions.left / conversionFactor),
        }
        entity.boundingBox = newBoundingBoxes
        entity.manuallyChanged = true
        dispatch('save', entity)
    }

</script>


<style>

    .bounding-box {
        position: absolute;
        padding: 10px;
    }

    .rectangle {
        width: 100%;
        height: 100%;
        border: 2px var(--red) solid;
    }

    button {
        position: absolute;
        left: 10px;
        top: -8px;

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
        top: 6px;
        left: calc(50% - 8px);
        width: 20px;
        height: 8px;
        cursor: row-resize;
    }

    #handle-right {
        right: 7px;
        top: calc(50% - 10px);
        width: 8px;
        height: 20px;
        cursor: col-resize;
    }

    #handle-bottom {
        bottom: 7px;
        left: calc(50% - 8px);
        width: 20px;
        height: 8px;
        cursor: row-resize;
    }

    #handle-left {
        left: 7px;
        top: calc(50% - 10px);
        width: 8px;
        height: 20px;
        cursor: col-resize;
    }

</style>
