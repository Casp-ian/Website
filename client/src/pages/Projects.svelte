<script>
    import ThisWebsite from "../projects/ThisWebsite.svelte";
    import Conways from "../projects/Conways.svelte";

    export let params;

    const projects = {
        "this": ThisWebsite,
        "conway": Conways,
    }

    let project = projects[params.id];

    if (!project) {
        // maybe just make this the latest project?
        project = projects["this"]
    }
</script>


<div id="projectHeader">
    {#each Object.entries(projects) as [name, component]}
        <a href={"/projects/" + name} on:click={() => {project = component}}>{name}</a>
    {/each}
</div>

<svelte:component this={project} />
