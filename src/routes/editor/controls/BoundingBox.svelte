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

    <p class="entity-label">{entity.label}</p>

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
    export let imageDimension: [number, number]  // width, height
    export let realDimension: [number, number]  // width, height
    export let mousePosition: [number, number]  // x, y viewport coordinates
    export let mouseUpCounter: number
    export let entity: Entity

    let box: HTMLDivElement

    let showEditButton = false
    let editMode: boolean

    $: conversionFactor = (realDimension[0] || 1) / (imageDimension[0] || 1)

    onMount(() => {
        drawBox(conversionFactor)
    })

    function drawBox(factor: number) {
        let width = Math.round((position.right - position.left) * factor)
        let height = Math.round((position.bottom - position.top) * factor)
        let top = Math.round(position.top * factor)
        let left = Math.round(position.left * factor)

        box.style.top = `${top}px`
        box.style.left = `${left}px`
        box.style.width = `${width}px`
        box.style.height = `${height}px`
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
        let boundingClientRect = box.getBoundingClientRect()

        switch (currentElement) {
            case 'top':
                let deltaTop = boundingClientRect.top - y
                box.style.height = `${box.offsetHeight + deltaTop}px`
                box.style.top = `${box.offsetTop - deltaTop}px`
                break
            case 'bottom':
                let deltaBottom = y - boundingClientRect.bottom
                box.style.height = `${box.offsetHeight + deltaBottom}px`
                break
            case 'left':
                let deltaLeft = boundingClientRect.left - x
                box.style.width = `${box.offsetWidth + deltaLeft}px`
                box.style.left = `${box.offsetLeft - deltaLeft}px`
                break
            case 'right':
                let deltaRight = x - boundingClientRect.right
                box.style.width = `${box.offsetWidth + deltaRight}px`
        }
    }

    function save() {
        editMode = false
        // let dimensions = box.getBoundingClientRect()
        let newBoundingBoxes = {
            top: Math.round(box.offsetTop / conversionFactor),
            right: Math.round((box.offsetLeft + box.offsetWidth) / conversionFactor),
            bottom: Math.round((box.offsetTop + box.offsetHeight) / conversionFactor),
            left: Math.round(box.offsetLeft / conversionFactor),
        }
        entity.boundingBox = newBoundingBoxes
        entity.manuallyChanged = true
        dispatch('save', entity)
    }

</script>


<style>

    .bounding-box {
        position: absolute;
        border: 1.5px var(--red) solid;
    }

    .entity-label {
        display: inline-block;
        position: relative;
        top: -19px;
        left: -1.5px;

        height: 12px;
        padding: 0 2px;

        font-size: 8px;
        color: var(--background-color);
        background-color: var(--red);

        border-radius: 2px;
    }

    button {
        position: absolute;
        top: -1px;
        left: -1px;

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
