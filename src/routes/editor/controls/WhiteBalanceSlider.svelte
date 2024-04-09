<div class="wrapper">
    <label for="slider">Weissabgleich</label>
    <div class="controls">
        <input on:change={(e) => update(e.target.value)} type="range" id="slider" value="{value}"
               min="{min}" max="{max}" step="{step}">
        <input on:change={(e) => update(e.target.value)} type="number" id="stepper" value={value}
               min="{min}" max="{max}" step="{step}">
    </div>
</div>


<script lang="ts">

    import {createEventDispatcher} from 'svelte'

    export let min: number
    export let max: number
    export let step: number
    export let initial: [number, number, number]

    let value = 1
    $: update(mapWbToValue(initial), false)

    const dispatch = createEventDispatcher()

    function update(newValue: Optional<number>, sendDispatch = true) {
        if (typeof newValue === 'undefined' || isNaN(newValue)) return

        value = newValue
        if (sendDispatch) {
            dispatch('update', {
                type: ['grading', 'whiteBalance'],
                value: mapValueToWb(value),
            })
        }
    }

    function mapWbToValue(wb: [number, number, number]): number {
        let redDelta = ((2) / (1.2 - 0.8)) * (wb[0] - 0.8)
        return Math.fround(redDelta)
    }

    function mapValueToWb(value: number): [number, number, number] {
        let red = 1 + (value - 1) / 5
        let blue = 1 - (value - 1) / 5
        return [red, 1, blue]
    }

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
        background: linear-gradient(to right, #51A1FF, #FFCF52, #FF6543);
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
