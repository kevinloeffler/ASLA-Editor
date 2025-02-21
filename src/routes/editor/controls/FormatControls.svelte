
<div class="wrapper">
    <div class="row">
        <p>Zuschneiden</p>
        <button on:click={toggle}>
            {#if resizing}
                Fertig
            {:else}
                Zuschneiden
            {/if}
        </button>
    </div>

    <label for="slider">Rotation</label>
    <div class="controls">
        <!--TODO-->
<!--        <input on:change={(e) => update(e.target.value)} type="range" id="slider" value="{value}"-->
<!--               min="{min}" max="{max}" step="{step}">-->
<!--        <input on:change={(e) => update(e.target.value)} type="number" id="stepper" value={value}-->
<!--               min="{min}" max="{max}" step="{step}">-->
    </div>
</div>


<script lang="ts">

    import {createEventDispatcher} from 'svelte'
    const dispatch = createEventDispatcher()

    let resizing = false

    function toggle() {
        resizing = !resizing
        dispatch( 'toggle', {resizing} )
    }

</script>


<style>

</style>
