<div>
    <div class="row">
<!--        <label for="{entity.label}">{entity.label}</label>-->
        <div class="select-wrapper">
            <img class="select-chevron" src="/assets/icons/editor-label-chevron.svg" alt="Chevron nach unten">
            <select class="label-select" bind:value={entity.label} on:change={save}>
                {#each Object.entries(ENTITIES) as [_, label]}
                    <option value="{label}" >{entityToText(label)}</option>
<!--
                    {#if label === entity.label}
                        <option value="{label}" selected>{entityToText(label)}</option>
                    {:else}
                        <option value="{label}" >{entityToText(label)}</option>
                    {/if}
-->
                {/each}
            </select>
        </div>

        <div>
            <button class="crop"><img src="/assets/icons/editor-crop-icon.svg" alt="Zuschneiden"></button>
            <button class="delete" on:click={deleteEntity}>
                <img src="/assets/icons/editor-delete-icon.svg" alt="Löschen">
            </button>
        </div>
    </div>

    <input id="{entity.label}" type="text" bind:value={entity.text} on:change={save}>
</div>


<script lang="ts">

    import {createEventDispatcher} from 'svelte'
    import {ENTITIES, entityToText} from '$lib/services/entity-service.js'
    const dispatch = createEventDispatcher()

    export let entity: Entity

    function save(_: Event) {
        entity.manuallyChanged = true
        dispatch('save', entity)
    }

    function deleteEntity() {
        dispatch('delete', {})
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
