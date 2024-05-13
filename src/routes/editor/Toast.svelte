
{#if visible}
    <div class="wrapper">
        <div class="toast">
            {#if msg}
                {msg}
            {/if}
            {#if hasAction}
                <button on:click={handleClick}>Abbrechen</button>
            {/if}
            <slot />
        </div>
    </div>
{/if}


<script lang="ts">

    let visible = false
    let msg = ''
    let hasAction = false
    let handleClick: CallableFunction = () => {}

    export function show(message: Optional<string>, action: Optional<CallableFunction>, duration: Optional<number>) {
        if (message) {
            msg = message
        }

        if (action) {
            hasAction = true
            handleClick = action
        }

        visible = true

        if (duration) {
            setTimeout(hide, duration)
        }
    }

    export function hide() {
        visible = false
    }

    // export function show(msg: string) {}

</script>


<style>

    .wrapper {
        position: fixed;
        bottom: 40px;
        right: 360px;
        left: 0;
        text-align: center;
    }

    .toast {
        display: inline;
        padding: 8px 12px;
        border-radius: 100px;
        font-size: 9pt;

        color: var(--background-color);
        background-color: var(--foreground-color);
    }

    button {
        border: none;
        border-radius: 100px;
        color: white;
        margin: 0 -8px 0 4px;
        position: relative;
        top: -1px;
        background-color: var(--red);
    }

</style>
