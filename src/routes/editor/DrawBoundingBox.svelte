{#if visible}
<div class="canvas"
     on:mousedown|stopPropagation={handleMouseDown}
     on:mouseup|stopPropagation={handleMouseUp}
     on:mousemove|stopPropagation={handleMouseMove}
     aria-hidden="true"
>
    <div class="bounding-box" class:hidden={!showBoundingBox} bind:this={boundingBox}></div>
</div>
{/if}

<script lang="ts">

    export let scalingFactor: number

    let entity: Optional<Entity>

    let resolveDrawPromise: ((entity: Entity) => void) | null = null
    let visible = false

    let boundingBox: HTMLDivElement
    let showBoundingBox = false
    let isDrawing = false

    export async function draw(ent: Entity): Promise<any> {
        console.log('start drawing. ent:', ent)
        entity = ent
        visible = true
        return new Promise<Entity>((resolve) => {
            resolveDrawPromise = resolve
        })
    }

    function handleMouseDown(e: MouseEvent) {
        showBoundingBox = true
        isDrawing = true
        boundingBox.style.left = `${e.offsetX}px`
        boundingBox.style.top = `${e.offsetY}px`
        boundingBox.style.width = '0'
        boundingBox.style.height = '0'
    }

    function handleMouseMove(e: MouseEvent) {
        if (!isDrawing) return
        boundingBox.style.width = `${e.offsetX - boundingBox.offsetLeft}px`
        boundingBox.style.height = `${e.offsetY - boundingBox.offsetTop}px`
    }

    function handleMouseUp() {
        isDrawing = false

        // save entity
        if (entity) {
            entity.hasBoundingBox = true
            entity.boundingBox.top = Math.round(boundingBox.offsetTop * scalingFactor)
            entity.boundingBox.left = Math.round(boundingBox.offsetLeft * scalingFactor)
            entity.boundingBox.right = Math.round((boundingBox.offsetLeft + boundingBox.offsetWidth) * scalingFactor)
            entity.boundingBox.bottom = Math.round((boundingBox.offsetTop + boundingBox.offsetHeight) * scalingFactor)
        }

        if (resolveDrawPromise) resolveDrawPromise(entity!)
        resolveDrawPromise = null
        visible = false
        showBoundingBox = false
        isDrawing = false
    }

</script>


<style>

    .canvas {
        position: absolute;
        top: 0;
        left: 0;
        width: 100%;
        height: 100%;

        display: flex;
        align-items: center;
        justify-content: center;
    }

    .hidden {
        display: none;
    }

    .bounding-box {
        position: absolute;
        border: solid 1.2px var(--red);
        width: 100px;
        height: 100px;
        pointer-events: none;
    }

</style>
