<script>
    import PrettyLink from '$components/PrettyLink.svelte';
    import Pane from "$components/Pane.svelte";
</script>

<Pane>
    <h1>This website</h1>
    <PrettyLink url="https://github.com/Casp-ian/Website">
        The source code is available on my github
    </PrettyLink>
    <p>
        I made this website to show of some cool projects i wanted to make, but the first project you
        would see would be the site itself.
    </p>
    <p>Right now it is also obviously unfinished.</p>
    <p>Written in svelte and for now no backend.</p>


</Pane>


<style>
    pre, p, h1, h2, h3 {
        padding: 1rem;
        margin: 0;
    }
</style>