<script lang="ts">
	import Cat from '$components/Cat.svelte';
	import WelcomeScreen from '$components/WelcomeScreen.svelte';
    import {projects} from '$lib/navigation';
    import Pane from "$components/Pane.svelte";
</script>

<Pane>
    <h1>Projects :)</h1>
    <p>
        projects ive made and want to show off and explain...
    </p>
</Pane>

{#each projects as project}
    <Pane>
        <h2>
            {project.name}
        </h2>
    </Pane>
{/each}


<style>
    pre, p, h1, h2, h3 {
        padding: 1rem;
        margin: 0;
    }
</style>
