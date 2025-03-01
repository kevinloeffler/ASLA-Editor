<div on:mouseenter={handleMouseEnter}
     on:mouseleave={handleMouseLeave}
     aria-hidden="true"
>
    <div class="row">
        <div class="select-wrapper">
            <img class="select-chevron" src="/assets/icons/editor-label-chevron.svg" alt="Chevron nach unten">
            <select class="label-select" bind:value={displayEntity.entity.label} bind:this={selectElement} on:change={save}>
                {#each Object.entries(ENTITIES) as [_, label]}
                    <option value="{label}" >{entityToText(label)}</option>
                {/each}
            </select>
        </div>

        <div>
            <button class="crop" on:click={updateHasBoundingBox}>
                {#if displayEntity.entity.hasBoundingBox}
                    <img src="/assets/icons/editor-crop-icon-delete.svg" alt="Zuschneiden">
                {:else}
                    <img src="/assets/icons/editor-crop-icon.svg" alt="Zuschneiden">
                {/if}
            </button>
            <button class="delete" on:click={deleteEntity}>
                <img src="/assets/icons/editor-delete-icon.svg" alt="Löschen">
            </button>
        </div>
    </div>

    <input class="entity-text-input" type="text" bind:value={displayEntity.entity.text} on:change={save}
           on:keydown={handleKeyDown}
    >
</div>


<script lang="ts">

    import {createEventDispatcher} from 'svelte'
    import {ENTITIES, entityToText} from '$lib/services/entity-service.js'
    const dispatch = createEventDispatcher()

    export let displayEntity: DisplayEntity

    let selectElement: HTMLSelectElement

    function save(_: Event) {
        displayEntity.entity.manuallyChanged = true
        dispatch('save', displayEntity.entity)
    }

    function deleteEntity() {
        dispatch('delete', {})
    }

    function updateHasBoundingBox() {
        if (displayEntity.entity.hasBoundingBox) {
            displayEntity.entity.hasBoundingBox = false
            dispatch('save')
        } else {
            dispatch('start-drawing', displayEntity)
        }
    }

    function handleMouseEnter() {
        displayEntity.highlight = true
        displayEntity = displayEntity
        dispatch('update', {entity: displayEntity})
    }

    function handleMouseLeave() {
        displayEntity.highlight = false
        displayEntity = displayEntity
        dispatch('update', {entity: displayEntity})
    }

    function handleKeyDown(e: KeyboardEvent) {
        // Keyboard Shortcuts for the different labels
        if (e.metaKey || e.ctrlKey) {
            if (e.key === 'o') {
                selectElement.value = 'LOC'
            } else if (e.key === 'b') {
                selectElement.value = 'CLT'
            } else if (e.key === 'm') {
                selectElement.value = 'MST'
            } else if (e.key === 'd' || e.key === 'e') {
                selectElement.value = 'DATE'
            } else if (e.key === 'c') {
                selectElement.value = 'CDATE'
            } else if (e.key === 'u') {
                selectElement.value = 'O'
            }
            selectElement.dispatchEvent(new Event('change'))
        }
    }


</script>


<style>

    .row {
        display: flex;
        justify-content: space-between;
        align-items: center;
    }

    .row > div {
        display: flex;
    }

    button {
        width: 32px;
        height: 24px;
        display: flex;
        align-items: center;
        justify-content: center;
        border-radius: 4px;
        border: none;
        margin-left: 4px;
        cursor: pointer;
    }

    .crop {
        background-color: var(--text-color);
    }

    .delete {
        background-color: var(--red);
    }

    input {
        border: none;
        border-radius: 4px;
        padding: 4px 8px;
        width: 100%;
        margin-top: 4px;
    }

    .label-select {
        appearance: none;
        background-color: transparent;
        border: none;
        padding-left: 16px;
        margin-left: -10px;
        z-index: 2;
    }

    .select-chevron {
        opacity: 0.33;
    }

</style>
