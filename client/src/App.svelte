<script>
    import router from 'page';

    import Home from "./pages/Home.svelte";
    import Projects from "./pages/Projects.svelte";
    import Blog from "./pages/Blog.svelte";
    import Error from "./pages/Error.svelte";

    import Header from "./components/Header.svelte";

    let page;
    let params;
    let currentRoute;

    router('/', (ctx) => {
        params = ctx.params;
        page = Home;
        currentRoute = '/';
    });
    router('/projects', (ctx) => {
        params = ctx.params;
        page = Projects;
        currentRoute = '/projects';
    });
    router('/projects/:id', (ctx) => {
        params = ctx.params;
        page = Projects;
        currentRoute = '/projects';
    });
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
    router('/error', (ctx) => {
        params = ctx.params;
        page = Error;
        currentRoute = '';
    });
    router('*', (ctx) => {
        params = ctx.params;
        page = Error;
        params.errorCode = 404;
    });
    router.start();
</script>


<main>
    <Header active={currentRoute}/>
    <svelte:component this={page} {params}/>
</main>


<style>
    :global(*) {
        --main-color: #171918;
        --back-color: #d8dcda;
        --interactable-color: #de494c;
        --accent-color: #4a3f63;
		    transition: background-color 0.2s;
    }
    :global(body.dark *) {
        --main-color: #d8dcda;
        --back-color: #171918;
        --interactable-color: #de494c;
        --accent-color: #4a3f63;
    }

    :root {
        font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen,
        Ubuntu, Cantarell, 'Open Sans', 'Helvetica Neue', sans-serif;
    }

    main {
        position: absolute;
        top: 0;
        min-height: 100vh;
        width: 100vw;
        background-color: var(--accent-color);
    }

    :global(p, h1, h2, h3, a, button) {
        color: var(--main-color);
    }
</style>
