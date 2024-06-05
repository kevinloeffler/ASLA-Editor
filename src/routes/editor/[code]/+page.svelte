<h1>Editor for: {project?.name}</h1>

<a href="/">Zurück</a>

<script lang="ts">
    import {page} from '$app/stores'
    import {STATE} from '$lib/services/state-manager'

    let code = $page.params.code
    let project = STATE.projects.get(code)

</script>