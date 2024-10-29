<script>
    import router from 'page';

    import { isVertical } from './lib/isMobile'

    // side bar things
    import Sidebar from './components/Sidebar.svelte';
    import Notifications from './components/Notifications.svelte';
    import Weather from './components/Weather.svelte';

    // pages
    import Home from "./pages/Home.svelte";
    import Blog from "./pages/Blog.svelte";
    import Error from "./pages/Error.svelte";

    // projects
    import ThisWebsite from './projects/ThisWebsite.svelte';
    import Cat from './projects/Cat.svelte';
    import TaggedFiles from './projects/TaggedFiles.svelte';
    import ImageToText from './projects/ImageToText.svelte';


    let page;
    let params;
    let currentRoute = '';

    router('/', (ctx) => {
        params = ctx.params;
        page = Home;
        currentRoute = '/home';
    });
    router('/home', (ctx) => {
        params = ctx.params;
        page = Home;
        currentRoute = '/home';
    });
    
    // ==============================
    router('/projects', (ctx) => {
        params = ctx.params;
        page = ThisWebsite;
        // this isnt really correct, but is the easiest way to make sure the correct thing gets underlined in the nav section
        currentRoute = '/projects/this';
    });
    router('/projects/this', (ctx) => {
        params = ctx.params;
        page = ThisWebsite;
        currentRoute = '/projects/this';
    });
    router('/projects/cat', (ctx) => {
        params = ctx.params;
        page = Cat;
        currentRoute = '/projects/cat';
    });
    router('/projects/tagged_files', (ctx) => {
        params = ctx.params;
        page = TaggedFiles;
        currentRoute = '/projects/tagged_files';
    });
    router('/projects/image_to_text', (ctx) => {
        params = ctx.params;
        page = ImageToText;
        currentRoute = '/projects/image_to_text';
    });
    
    // ==============================
    router('/blog', (ctx) => {
        params = ctx.params;
        page = Blog;
        currentRoute = '/blog';
    });
    router('/blog/:id', (ctx) => {
        params = ctx.params;
        page = Blog;
        currentRoute = '/blog';
    });

    // ==============================
    router('/error', (ctx) => {
        params = ctx.params;
        page = Error;
        currentRoute = '/error';
    });
    router('*', (ctx) => {
        params = ctx.params;
        page = Error;
        params.errorCode = 404;
        params.message = 'That page does not exist';
        currentRoute = '/error';
    });
    router.start();
</script>


<main>
    {#if isVertical()}
    <Notifications/>
    <Weather/>
    <svelte:component this={page} {params}/>
    {:else}
    <div id="sidebar">
        <Sidebar active={currentRoute}/>
    </div>
    
    <div id="page">
        <svelte:component this={page} {params}/>
    </div>
    
    <div id="extra">
        <Notifications/>
        <Weather/>
    </div>
    {/if}
</main>


<style>
    :global(*) {
        --main-color: #F7FFF7;
        --back-color: #240021;
        --back-color2: #3D003A;
        --interactable-color: #ED455C;
        --accent-color: #8338EC;
		    transition: background-color 0.2s;
    }

    :global(p, h1, h2, h3, button, pre) {
        color: var(--main-color);
        overflow: hidden;
        overflow-wrap: break-word;
    }

    :global(a) {
        color: var(--interactable-color);
    }

    :root {
        font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen,
        Ubuntu, Cantarell, 'Open Sans', 'Helvetica Neue', sans-serif;
    }

    main {
        display: flex;
        flex-direction: row;
        flex-wrap: nowrap;

        position: absolute;
        top: 0;
        min-height: 100vh;
        width: 100vw;
        background-color: var(--back-color);
    }

    #sidebar {
        width: 25vw;
    }

    #page {
        box-shadow: 0 0 .5rem var(--back-color2);
        width: 50vw;
    }

    #extra {
        width: 25vw;
    }
</style>
