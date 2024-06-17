
<div class="bounding-box" class:bounding-box-highlight={displayEntity.highlight} bind:this={boundingBox}>

    <div class="label">{displayEntity.entity.label}</div>

    <div class="handle-area" id="top-left" class:handle-area-current={currentArea === 'tl'}
         on:mouseup={() => {isDragging = false; currentArea = ''}}
         on:mousemove={dragTopLeft}
         on:mouseleave={() => {isDragging = false; currentArea = ''}}
         aria-hidden="true"
    >
        <div class="handle"
             on:mousedown|stopPropagation={(e) => {isDragging = true; dragTopLeft(e)}}
             aria-hidden="true"
        ></div>
    </div>

    <div class="handle-area" id="top-right" class:handle-area-current={currentArea === 'tr'}
         on:mouseup={() => {isDragging = false; currentArea = ''}}
         on:mousemove={dragTopRight}
         on:mouseleave={() => {isDragging = false; currentArea = ''}}
         aria-hidden="true"
    >
        <div class="handle"
             on:mousedown|stopPropagation={(e) => {isDragging = true; dragTopRight(e)}}
             aria-hidden="true"
        ></div>
    </div>

    <div class="handle-area" id="bottom-left" class:handle-area-current={currentArea === 'bl'}
         on:mouseup={() => {isDragging = false; currentArea = ''}}
         on:mousemove={dragBottomLeft}
         on:mouseleave={() => {isDragging = false; currentArea = ''}}
         aria-hidden="true"
    >
        <div class="handle"
             on:mousedown|stopPropagation={(e) => {isDragging = true; dragBottomLeft(e)}}
             aria-hidden="true"
        ></div>
    </div>

    <div class="handle-area" id="bottom-right" class:handle-area-current={currentArea === 'br'}
         on:mouseup={() => {isDragging = false; currentArea = ''}}
         on:mousemove={dragBottomRight}
         on:mouseleave={() => {isDragging = false; currentArea = ''}}
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

    export let displayEntity: DisplayEntity
    export let scaleFactor: number
    let isDragging = false

    let oldDisplayEntity = {...displayEntity}

    let boundingBox: HTMLDivElement

    onMount( () => render(scaleFactor) )

    $: render(scaleFactor)

    let currentArea = ''

    function render(scale: number) {
        if (!boundingBox) return

        let top = Math.round(displayEntity.entity.boundingBox.top / scale)
        let left = Math.round(displayEntity.entity.boundingBox.left / scale)
        let width = Math.round((displayEntity.entity.boundingBox.right - displayEntity.entity.boundingBox.left) / scale)
        let height = Math.round((displayEntity.entity.boundingBox.bottom - displayEntity.entity.boundingBox.top) / scale)

        boundingBox.style.top = `${top}px`
        boundingBox.style.left = `${left}px`
        boundingBox.style.height = `${height}px`
        boundingBox.style.width = `${width}px`
    }

    function dragTopLeft(e: MouseEvent) {
        if (!isDragging) return
        currentArea = 'tl'
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
        currentArea = 'tr'
        let clientRect = boundingBox.getBoundingClientRect()
        let deltaX = Math.round((e.clientX - clientRect.right) / 3)
        let deltaY = Math.round((e.clientY - clientRect.top) / 3)
        boundingBox.style.width = `${boundingBox.offsetWidth + deltaX}px`
        boundingBox.style.height = `${boundingBox.offsetHeight - deltaY}px`
        boundingBox.style.top = `${boundingBox.offsetTop + deltaY}px`
    }

    function dragBottomRight(e: MouseEvent) {
        if (!isDragging) return
        currentArea = 'br'
        let clientRect = boundingBox.getBoundingClientRect()
        let deltaX = Math.round((e.clientX - clientRect.right) / 3)
        let deltaY = Math.round((e.clientY - clientRect.bottom) / 3)
        boundingBox.style.width = `${boundingBox.offsetWidth + deltaX}px`
        boundingBox.style.height = `${boundingBox.offsetHeight + deltaY}px`
    }

    function dragBottomLeft(e: MouseEvent) {
        if (!isDragging) return
        currentArea = 'bl'
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

        const deltaTop = displayEntity.entity.boundingBox.top - boundingBox.offsetTop * scaleFactor
        const deltaRight = displayEntity.entity.boundingBox.right - (boundingBox.offsetLeft + boundingBox.offsetWidth) * scaleFactor
        const deltaBottom = displayEntity.entity.boundingBox.bottom - (boundingBox.offsetTop + boundingBox.offsetHeight) * scaleFactor
        const deltaLeft = displayEntity.entity.boundingBox.left - boundingBox.offsetLeft * scaleFactor

        if (Math.abs(deltaTop) < margin &&
            Math.abs(deltaRight) < margin &&
            Math.abs(deltaBottom) < margin &&
            Math.abs(deltaLeft) < margin) return

        displayEntity.entity.boundingBox.top = Math.round(
            displayEntity.entity.boundingBox.top - (Math.abs(deltaTop) > margin ? deltaTop : 0))
        displayEntity.entity.boundingBox.right = Math.round(
            displayEntity.entity.boundingBox.right - (Math.abs(deltaRight) > margin ? deltaRight : 0))
        displayEntity.entity.boundingBox.bottom = Math.round(
            displayEntity.entity.boundingBox.bottom - (Math.abs(deltaBottom) > margin ? deltaBottom : 0))
        displayEntity.entity.boundingBox.left = Math.round(
            displayEntity.entity.boundingBox.left- (Math.abs(deltaLeft) > margin ? deltaLeft : 0))

        let dispatch = createEventDispatcher()
        dispatch('save', {
            newDisplayEntity: displayEntity,
            oldDisplayEntity: oldDisplayEntity,
        })
        oldDisplayEntity = displayEntity  // should not be necessary i think???
    }

</script>


<style>

    .bounding-box {
        position: absolute;
        border: 1.2px solid var(--red);
    }

    .bounding-box-highlight {
        box-shadow: var(--red) 0 0 20px;
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
        border: 2px solid var(--red);
        background-color: white;
        cursor: grab;
    }

    .handle-area-current {
        width: 60px;
        height: 60px;
        z-index: 9;
    }

    .handle-area-current#bottom-right {
        bottom: -30px;
        right: -30px;
    }

    .handle-area-current#top-left {
        top: -30px;
        left: -30px;
    }

    .handle-area-current#top-right {
        top: -30px;
        right: -30px;
    }

    .handle-area-current#bottom-left {
        bottom: -30px;
        left: -30px;
    }

</style>
