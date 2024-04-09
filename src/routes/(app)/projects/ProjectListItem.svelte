<a class="project-wrapper" href="/projects/{code}">
    <p class="code">{code}</p>
    <p class="project">{project}</p>
    <p class="progress-text">{imagesDone}&nbsp/&nbsp{imagesTotal}</p>
    <div class="progress-bar-wrapper">
        <div class="progress-bar" bind:this={progressBar}></div>
    </div>
</a>


<script lang="ts">

    import {onMount} from 'svelte'

    export let code: string
    export let project: string
    export let imagesDone: number
    export let imagesTotal: number

    let progressBar: HTMLDivElement

    onMount(() => {
        // set progress bar width
        progressBar.style.width = `${100 / imagesTotal * imagesDone}%`
    })

</script>


<style>

    .project-wrapper {
        display: flex;
        align-items: center;
        background-color: var(--raised-color);
        border-radius: 8px;
        padding: 8px 12px;
        margin-bottom: 4px;
        overflow: hidden;
        transition-duration: 200ms;
    }

    .project-wrapper:hover {
        background-color: var(--raised-hover-color);
    }

    .code {
        width: 50px;
        font-weight: 600;
    }

    .project {
        width: 60%;
        max-width: 260px;
    }

    .progress-text {
        width: 80px;
    }

    .progress-bar-wrapper {
        width: 40%;
        min-width: 40px;
        max-width: 100px;

        height: 8px;
        border-radius: 8px;
        background-color: #D1D1D1;
        margin-left: 8px;
        overflow: hidden;
    }

    .progress-bar {
        height: 100%;
        background-color: var(--green);
    }

</style>
