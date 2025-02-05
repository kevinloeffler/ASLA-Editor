{#if position}
    <div class="target" on:mouseleave={handleMouseLeave} style="left:{position?.x || 0}px; top:{position?.y || 0}px;" role="dialog">
        <div class="wrapper">
            {#each Object.values(ENTITIES) as ent}
                <button on:click={() => handleButtonClick(ent)} class="selection">{entityToText(ent)}</button>
            {/each}
        </div>
    </div>
{/if}


<script lang="ts">

    import {ENTITIES, entityToText} from '$lib/services/entity-service'

    export let entity: Entity
    export let position: Optional<{ x: number; y: number }>
    export let drawUpdate: () => void

    function handleMouseLeave() {
        position = undefined
    }

    function handleButtonClick(ent: string) {
        console.log('ent')
        entity.label = ent
        position = undefined
        drawUpdate()
        // Focus the most recent text input
        let elements: NodeListOf<HTMLInputElement> = document.querySelectorAll('.entity-text-input')
        if (elements.length > 0) {
            elements[elements.length - 1].focus()
        }
    }

</script>


<style>

    .target {
        position: absolute;
        margin-top: -20px;
        margin-left: -20px;
        padding: 20px;
    }

    .wrapper {
        display: flex;
        flex-direction: column;
        background-color: var(--raised-color);
        border-radius: 8px;
        overflow: hidden;
        box-shadow: 0 0 30px rgba(0, 0, 0, 0.25);
    }

    .selection:first-child {
        padding-top: 12px;
    }

    .selection:last-child {
        padding-bottom: 12px;
    }

    .selection {
        background-color: transparent;
        border: none;
        padding: 6px 16px;
        width: 100%;
        text-align: left;
        cursor: pointer;
    }

    .selection:hover {
        background-color: var(--red);
    }

</style>
