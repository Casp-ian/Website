<script>
    export let active;
</script>


<!--header component is not reusable right now, but maybe its not always worth the time to make a component reusable that only gets used once per project-->
<nav>
    <ul>
        <li><h1>Website</h1></li>
        <li><a class:active={active === "/"} href="/">Home</a></li>
        <li><a class:active={active === "/projects"} href="/projects">Projects</a></li>
        <!-- <li><a class:active={active === "/blog"} href="/blog">Blog</a></li> -->
    </ul>
    <button on:click={() => window.document.body.classList.toggle('dark')}>
        totally finished darkmod button
    </button>
</nav>


<style>
    nav {
        height: 4rem;
    
        text-align: center;
        background-color: var(--back-color);
        margin: .3rem;
        border-radius: .3rem;
    }

    ul, li {
        display: inline-block;
        list-style-type: none;
        padding: 0 1em;
    }

    h1 {
        margin: 0;
    }

    a {
        text-decoration: none;
    }

    a.active {
        font-weight: bold;
    }

    button {
        float: right;
        background-color: var(--interactable-color);
    }
</style>
