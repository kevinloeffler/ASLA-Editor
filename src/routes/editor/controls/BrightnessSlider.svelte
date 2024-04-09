<div class="wrapper">
    <label for="slider">Helligkeit</label>
    <div class="controls">
        <input type="range" id="slider" bind:value="{value}" min="{min}" max="{max}" step="{step}">
        <input type="number" id="stepper" bind:value={value} min="{min}" max="{max}" step="{step}">
    </div>
</div>


<script lang="ts">

    import {createEventDispatcher} from 'svelte'

    export let min: number
    export let max: number
    export let step: number
    export let initial: number

    // let value = initial
    $: value = initial

    const dispatch = createEventDispatcher()

    $: dispatch('update', {
        type: ['grading', 'brightness'],
        value: value,
    })

</script>


<style>

    .controls {
        display: flex;
        align-items: center;
    }

    #stepper {
        padding: 8px 8px;
        margin-left: 8px;
        border: none;
        border-radius: 8px;
        text-align: center;
    }

     #slider {
        -webkit-appearance: none;
        width: 100%;
        height: 7px;
        border-radius: 100px;
        background: linear-gradient(to right, var(--text-color), var(--raised-color));
        outline: none;
        margin: 10px 0;
    }

     #slider::-webkit-slider-thumb {
         -webkit-appearance: none;
         appearance: none;
         width: 28px;
         height: 28px;
         border-radius: 100px;
         background-color: white;
         filter: drop-shadow(1px 4px 4px rgba(0, 0, 0, 0.33));
         cursor: pointer;
     }

</style>
