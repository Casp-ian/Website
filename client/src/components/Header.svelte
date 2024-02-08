<script>
    export let active;
</script>


<!--header component is not reusable right now, but maybe its not always worth the time to make a component reusable that only gets used once per project-->
<nav>
    <ul>
        <li><h1>Website</h1></li>
        <li><a class:active={active === "/"} href="/">Home</a></li>
        <li><a class:active={active === "/projects"} href="/projects">Projects</a></li>
        <li><a class:active={active === "/blog"} href="/blog">Blog</a></li>
    </ul>
</nav>


<style>
    nav {
        height: calc(4rem - 1px); /*1px is border*/
        border-bottom: 1px solid black;

        text-align: center;
        margin: 0;
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
        color: black;
    }

    a.active {
        font-weight: bold;
    }
</style>