<script>
    import ThisWebsite from "../projects/ThisWebsite.svelte";
    import Conways from "../projects/Conways.svelte";
    import Orbit from "../projects/Orbit.svelte";
    import TaggedFiles from "../projects/TaggedFiles.svelte";

    export let params;

    const projects = {
        "this": ThisWebsite,
        // "conway": Conways,
        // "orbiting balls": Orbit,
        "tagged files": TaggedFiles,
    }

    let project = projects[params.id];

    if (!project) {
        project = projects["this"]
    }
</script>


<div id="projectHeader">
    {#each Object.entries(projects) as [name, component]}
        <a href={"/projects/" + name} on:click={() => {project = component}}>{name}</a>
    {/each}
</div>

<svelte:component this={project} />


<style>
    #projectHeader {
        display: flex;
        flex-direction: row;
        justify-content: flex-start;
        align-items: center;

        padding-left: 2rem;
    }

    #projectHeader>a {
        background-color: var(--back-color);
        margin-left: .1rem;
        padding: .5rem;

        text-decoration: none;

        margin: .3rem;
        border-radius: .3rem;
    }
</style>
