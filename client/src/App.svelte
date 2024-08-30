<script>
    import router from 'page';

    import Home from "./pages/Home.svelte";
    import Blog from "./pages/Blog.svelte";
    import Error from "./pages/Error.svelte";

    import Header from "./components/Header.svelte";
    import Footer from "./components/Footer.svelte";
    import Notifications from './components/Notifications.svelte';
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
        currentRoute = '/projects';
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
    <div id="sidebar">
        <Header active={currentRoute}/>
        <Footer/>
    </div>
    
    <div id="page">
        <svelte:component this={page} {params}/>
    </div>
    
    <div id="extra">
        <Notifications/>
    </div>
</main>


<style>
    :global(*) {
        --main-color: #101816;
        --back-color: #EDCB96;
        --interactable-color: #295F98;
        --accent-color: #667BC6;
		    transition: background-color 0.2s;
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
        box-shadow: 0 0 2rem var(--main-color);
        width: 50vw;
    }

    #extra {
        width: 25vw;
    }

    :global(p, h1, h2, h3, a, button) {
        color: var(--main-color);
    }
</style>
