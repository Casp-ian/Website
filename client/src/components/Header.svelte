<script>
    export let active;
    const mainPaths = {
        "home": "/home",
        "projects": "/projects",
    }
    
    const projects = {
        "this": "/projects/this",
        "cat": "/projects/cat",
        // "conway": "/projects/conway",
        // "orbiting balls": "/projects/orbiting_balls",
        "tagged files": "/projects/tagged_files",
        "image to text": "/projects/image_to_text"
    }
</script>


<!--header component is not reusable right now, but maybe its not always worth the time to make a component reusable that only gets used once per project-->
<nav>
    {#each Object.entries(mainPaths) as [name, component]}
        <a href={component} class:active={active.startsWith(component)}>{name}</a>
    {/each}
</nav>

{#if active.startsWith("/projects")}
<nav>
    {#each Object.entries(projects) as [name, component]}
        <a href={component} class:active={active === component}>{name}</a>
    {/each}
</nav>
{/if}


<style>
    nav {
        display: flex;
        flex-direction: row;
    
        border-radius: .3rem;

        width: 100%;
    }

    a {
        padding: .2rem;
        text-decoration: none;
    }

    a.active {
        font-weight: bold;
    }
</style>
