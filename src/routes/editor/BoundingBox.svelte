
<div class="bounding-box" bind:this={boundingBox}>

    <div class="label">{entity.label}</div>

    <div class="handle-area" id="top-left"
         on:mouseup={() => isDragging = false}
         on:mousemove={dragTopLeft}
         on:mouseleave={() => isDragging = false}
         aria-hidden="true"
    >
        <div class="handle"
             on:mousedown|stopPropagation={(e) => {isDragging = true; dragTopLeft(e)}}
             aria-hidden="true"
        ></div>
    </div>

    <div class="handle-area" id="top-right"
         on:mouseup={() => isDragging = false}
         on:mousemove={dragTopRight}
         on:mouseleave={() => isDragging = false}
         aria-hidden="true"
    >
        <div class="handle"
             on:mousedown|stopPropagation={(e) => {isDragging = true; dragTopRight(e)}}
             aria-hidden="true"
        ></div>
    </div>

    <div class="handle-area" id="bottom-left"
         on:mouseup={() => isDragging = false}
         on:mousemove={dragBottomLeft}
         on:mouseleave={() => isDragging = false}
         aria-hidden="true"
    >
        <div class="handle"
             on:mousedown|stopPropagation={(e) => {isDragging = true; dragBottomLeft(e)}}
             aria-hidden="true"
        ></div>
    </div>

    <div class="handle-area" id="bottom-right"
         on:mouseup={() => isDragging = false}
         on:mousemove={dragBottomRight}
         on:mouseleave={() => isDragging = false}
         aria-hidden="true"
    >
        <div class="handle"
             on:mousedown|stopPropagation={(e) => {isDragging = true; dragBottomRight(e)}}
             aria-hidden="true"
        ></div>
    </div>

</div>


<script lang="ts">

    import {createEventDispatcher, onMount} from 'svelte'

    export let entity: Entity
    export let scaleFactor: number
    let isDragging = false

    let boundingBox: HTMLDivElement

    onMount( () => render(scaleFactor) )

    $: render(scaleFactor)

    function render(scale: number) {
        if (!boundingBox) return

        let top = Math.round(entity.boundingBox.top / scale)
        let left = Math.round(entity.boundingBox.left / scale)
        let width = Math.round((entity.boundingBox.right - entity.boundingBox.left) / scale)
        let height = Math.round((entity.boundingBox.bottom - entity.boundingBox.top) / scale)

        boundingBox.style.top = `${top}px`
        boundingBox.style.left = `${left}px`
        boundingBox.style.height = `${height}px`
        boundingBox.style.width = `${width}px`
    }

    function dragTopLeft(e: MouseEvent) {
        if (!isDragging) return
        let clientRect = boundingBox.getBoundingClientRect()
        let deltaX = Math.round((e.clientX - clientRect.left) / 3)
        let deltaY = Math.round((e.clientY - clientRect.top) / 3)
        boundingBox.style.width = `${boundingBox.offsetWidth - deltaX}px`
        boundingBox.style.left = `${boundingBox.offsetLeft + deltaX}px`
        boundingBox.style.height = `${boundingBox.offsetHeight - deltaY}px`
        boundingBox.style.top = `${boundingBox.offsetTop + deltaY}px`
    }

    function dragTopRight(e: MouseEvent) {
        if (!isDragging) return
        let clientRect = boundingBox.getBoundingClientRect()
        let deltaX = Math.round((e.clientX - clientRect.right) / 3)
        let deltaY = Math.round((e.clientY - clientRect.top) / 3)
        boundingBox.style.width = `${boundingBox.offsetWidth + deltaX}px`
        boundingBox.style.height = `${boundingBox.offsetHeight - deltaY}px`
        boundingBox.style.top = `${boundingBox.offsetTop + deltaY}px`
    }

    function dragBottomRight(e: MouseEvent) {
        if (!isDragging) return
        let clientRect = boundingBox.getBoundingClientRect()
        let deltaX = Math.round((e.clientX - clientRect.right) / 3)
        let deltaY = Math.round((e.clientY - clientRect.bottom) / 3)
        boundingBox.style.width = `${boundingBox.offsetWidth + deltaX}px`
        boundingBox.style.height = `${boundingBox.offsetHeight + deltaY}px`
    }

    function dragBottomLeft(e: MouseEvent) {
        if (!isDragging) return
        let clientRect = boundingBox.getBoundingClientRect()
        let deltaX = Math.round((e.clientX - clientRect.left) / 3)
        let deltaY = Math.round((e.clientY - clientRect.bottom) / 3)
        boundingBox.style.width = `${boundingBox.offsetWidth - deltaX}px`
        boundingBox.style.left = `${boundingBox.offsetLeft + deltaX}px`
        boundingBox.style.height = `${boundingBox.offsetHeight + deltaY}px`
    }

    $: isDragging || save()

    function save() {
        if (!boundingBox) return
        const margin = 3  // how many pixel we account for as rounding error in the conversion

        const deltaTop = entity.boundingBox.top - boundingBox.offsetTop * scaleFactor
        const deltaRight = entity.boundingBox.right - (boundingBox.offsetLeft + boundingBox.offsetWidth) * scaleFactor
        const deltaBottom = entity.boundingBox.bottom - (boundingBox.offsetTop + boundingBox.offsetHeight)  * scaleFactor
        const deltaLeft = entity.boundingBox.left - boundingBox.offsetLeft * scaleFactor

        if (Math.abs(deltaTop) < margin &&
            Math.abs(deltaRight) < margin &&
            Math.abs(deltaBottom) < margin &&
            Math.abs(deltaLeft) < margin) return

        entity.boundingBox.top = Math.round(entity.boundingBox.top - (Math.abs(deltaTop) > margin ? deltaTop : 0))
        entity.boundingBox.right = Math.round(entity.boundingBox.right - (Math.abs(deltaRight) > margin ? deltaRight : 0))
        entity.boundingBox.bottom = Math.round(entity.boundingBox.bottom - (Math.abs(deltaBottom) > margin ? deltaBottom : 0))
        entity.boundingBox.left = Math.round(entity.boundingBox.left- (Math.abs(deltaLeft) > margin ? deltaLeft : 0))

        let dispatch = createEventDispatcher()
        dispatch('save', entity)
    }

</script>


<style>

    .bounding-box {
        position: absolute;
        border: 1.2px solid var(--red);
    }

    .label {
        position: absolute;
        top: -6px;
        left: -1px;

        display: inline;
        padding: 0 2px;
        font-size: 3pt;
        color: white;
        background-color: var(--red);
    }

    /* Handles */

    #top-left {
        top: -9px;
        left: -9px;
    }

    #top-right {
        top: -9px;
        right: -9px;
    }

    #bottom-left {
        bottom: -9px;
        left: -9px;
    }

    #bottom-right {
        bottom: -9px;
        right: -9px;
    }

    .handle-area {
        position: absolute;
        width: 16px;
        height: 16px;
        display: flex;
        align-items: center;
        justify-content: center;
    }

    .handle-area:hover > .handle {
        width: 6px;
        height: 6px;
        border-radius: 100%;
        background-color: var(--red);
        cursor: grab;
    }

</style>
