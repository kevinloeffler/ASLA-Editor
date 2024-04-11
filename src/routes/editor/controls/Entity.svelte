<div>
    <div class="row">
        <label for="{entity.label}">{entity.label}</label>
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
    const dispatch = createEventDispatcher()

    export let entity: Entity

    function save(_: InputEvent) {
        entity.manuallyChanged = true
        dispatch('save', entity)
    }

    function deleteEntity() {
        console.log('dispatching delete')
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

</style>
