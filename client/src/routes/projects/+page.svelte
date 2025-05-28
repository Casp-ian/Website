<script lang="ts">
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
        <a href={project.path}>Link</a>
        <p>Hihi</p>
    </Pane>
{/each}


<style>
    a, pre, p, h1, h2, h3 {
        padding: 1rem;
        margin: 0;
    }
</style>
