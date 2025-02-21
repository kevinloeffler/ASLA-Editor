<div class="overlay" bind:this={overlay}>
    {#if resizing}
        <div class="handle-area top-left" on:mousedown={(e) => handleMouseDown(e, [Handle.Top, Handle.Left])} aria-hidden="true">
            <div class="handle">
                <img src="/assets/icons/resize-handle-corner.svg" alt="">
            </div>
        </div>
        <div class="handle-area top" on:mousedown={(e) => handleMouseDown(e, [Handle.Top])} aria-hidden="true">
            <div class="handle">
                <img src="/assets/icons/resize-handle-side.svg" alt="">
            </div>
        </div>
        <div class="handle-area top-right" on:mousedown={(e) => handleMouseDown(e, [Handle.Top, Handle.Right])} aria-hidden="true">
            <div class="handle">
                <img src="/assets/icons/resize-handle-corner.svg" alt="">
            </div>
        </div>
        <div class="handle-area right" on:mousedown={(e) => handleMouseDown(e, [Handle.Right])} aria-hidden="true">
            <div class="handle">
                <img src="/assets/icons/resize-handle-side.svg" alt="">
            </div>
        </div>
        <div class="handle-area bottom-right" on:mousedown={(e) => handleMouseDown(e, [Handle.Bottom, Handle.Right])}
             aria-hidden="true">
            <div class="handle">
                <img src="/assets/icons/resize-handle-corner.svg" alt="">
            </div>
        </div>
        <div class="handle-area bottom" on:mousedown={(e) => handleMouseDown(e, [Handle.Bottom])} aria-hidden="true">
            <div class="handle">
                <img src="/assets/icons/resize-handle-side.svg" alt="">
            </div>
        </div>
        <div class="handle-area bottom-left" on:mousedown={(e) => handleMouseDown(e, [Handle.Bottom, Handle.Left])} aria-hidden="true">
            <div class="handle">
                <img src="/assets/icons/resize-handle-corner.svg" alt="">
            </div>
        </div>
        <div class="handle-area left" on:mousedown={(e) => handleMouseDown(e, [Handle.Left])} aria-hidden="true">
            <div class="handle">
                <img src="/assets/icons/resize-handle-side.svg" alt="">
            </div>
        </div>
    {/if}
</div>


<script lang="ts">

    import {createEventDispatcher, onMount} from 'svelte'
    const dispatch = createEventDispatcher()

    enum Handle {
        Top = 'top',
        Right = 'right',
        Bottom = 'bottom',
        Left = 'left',
    }

    export let metadata: Metadata
    export let resizing: boolean
    export let scaleTranslationFactor: number
    export let scale: number

    let overlay: HTMLDivElement

    onMount(() => {
        redrawOverlay(metadata?.format?.crop, scaleTranslationFactor)
    })

    $: redrawOverlay(metadata?.format?.crop, scaleTranslationFactor)

    function redrawOverlay(crop: any, scale: number) {
        if (!overlay || !crop) return
        overlay.style.top = Math.round(crop.top / scale) + 'px'
        overlay.style.right = Math.round(crop.right / scale) + 'px'
        overlay.style.bottom = Math.round(crop.bottom / scale) + 'px'
        overlay.style.left = Math.round(crop.left / scale) + 'px'
    }

    function handleMouseDown(event: MouseEvent, handle: Handle[]) {
        if (resizing) {
            event.stopPropagation()

            const resizeHandler = (e: MouseEvent) => resize(e, handle)
            const cleanupHandler = () => overlay.removeEventListener('mousemove', resizeHandler)

            overlay.addEventListener('mousemove', resizeHandler)
            document.addEventListener('mouseup', cleanupHandler)
        }
    }

    function resize(event: MouseEvent, handles: Handle[]) {
        if (!metadata?.format?.crop) return

        if (handles.includes(Handle.Top)) {
            metadata.format.crop.top = Math.round(
                metadata.format.crop.top + event.movementY * scaleTranslationFactor / scale
            )
        }
        if (handles.includes(Handle.Right)) {
            metadata.format.crop.right = Math.round(
                metadata.format.crop.right - event.movementX * scaleTranslationFactor / scale
            )
        }
        if (handles.includes(Handle.Bottom)) {
            metadata.format.crop.bottom = Math.round(
                metadata.format.crop.bottom - event.movementY * scaleTranslationFactor / scale
            )
        }
        if (handles.includes(Handle.Left)) {
            metadata.format.crop.left = Math.round(
                metadata.format.crop.left + event.movementX * scaleTranslationFactor / scale
            )
        }
        dispatch('resize', metadata)
    }

</script>


<style>

    .overlay {
        position: absolute;
        background: transparent;
        box-shadow: 0 0 0 9999px rgba(255, 255, 255, 0.66);
        border: 3px solid white;
        pointer-events: none;
    }

    .handle-area {
        position: absolute;
        width: 60px;
        pointer-events: all;
        padding: 10px;
    }

    img {
        width: 100%;
        pointer-events: none;
    }

    .top-left {
        top: -14px;
        left: -14px;
        transform: rotate(-90deg);
    }

    .top-left > .handle {
        cursor: nwse-resize;
    }

    .top {
        top: -14px;
        left: calc(50% - 25px);
    }

    .top > .handle {
        cursor: ns-resize;
    }

    .top-right {
        top: -14px;
        right: -14px;
    }

    .top-right > .handle {
        cursor: nesw-resize;
    }

    .right {
        right: -30px;
        top: calc(50% - 25px);
        transform: rotate(90deg);
    }

    .right > .handle {
        cursor: ew-resize;
    }

    .bottom-right {
        bottom: -14px;
        right: -14px;
        transform: rotate(90deg);
    }

    .bottom-right > .handle {
        cursor: nwse-resize;
    }

    .bottom {
        bottom: -14px;
        left: calc(50% - 25px);
    }

    .bottom > .handle {
        cursor: ns-resize;
    }

    .bottom-left {
        bottom: -14px;
        left: -14px;
        transform: rotate(180deg);
    }

    .bottom-left > .handle {
        cursor: nesw-resize;
    }

    .left {
        left: -30px;
        top: calc(50% - 25px);
        transform: rotate(90deg);
    }

    .left > .handle {
        cursor: ew-resize;
    }

</style>
