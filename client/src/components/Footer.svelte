<script>
    import PrettyLink from "./PrettyLink.svelte";
</script>

<hr>
<div id="links">
    <PrettyLink url="https://github.com/Casp-ian">
        github
    </PrettyLink>
    <PrettyLink url="https://www.youtube.com/watch?v=dQw4w9WgXcQ" >
        i have no other links :)
    </PrettyLink>
</div>

<style>
    #links {
        display: flex;
        flex-direction: row;
        align-items: center;
        justify-content: space-evenly;
    }
</style>
