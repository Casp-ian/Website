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
        "image to text": "/projects/image_to_text",
        // "tanks": "/projects/tanks",
    }
</script>


<div id="wrapper">
    <nav id="main">
        {#each Object.entries(mainPaths) as [name, component]}
            <a href={component} class:active={active.startsWith(component)}>{name}</a>
        {/each}
    </nav>
    
    {#if active.startsWith("/projects")}
    <nav id="projects">
        <!-- TODO make nice icon, also, not sure if this 'back arrow' looks good -->
        <a href="/home">&lt=</a>
        {#each Object.entries(projects) as [name, component]}
            <a href={component} class:active={active === component}>{name}</a>
        {/each}
    </nav>
    {/if}
</div>


<style>
    #wrapper {
        display: flex;
        flex-direction: row;
            
        padding: 1rem;
    }
    
    nav {
        display: flex;
        flex-direction: column;

        border-radius: .3rem;

        width: 100%;
    }

    a {
        text-align: right;
        padding: .2rem;
        text-decoration: none;
    }

    a.active {
        text-decoration: underline;
    }
</style>
