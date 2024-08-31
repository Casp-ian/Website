<script>
    import PrettyLink from "./PrettyLink.svelte";
</script>

<div id="links">
    <hr>
    <PrettyLink url="https://github.com/Casp-ian">
        github
    </PrettyLink>
    <PrettyLink url="https://www.youtube.com/watch?v=dQw4w9WgXcQ" >
        i have no other links :)
    </PrettyLink>
</div>

<style>
    hr {
        color: var(--back-color2);
        width: 100%;
    }
    
    #links {
        position: absolute;
        bottom: 0;
        
        padding: 1rem;
        display: flex;
        flex-direction: column;
        justify-content: space-evenly;
    }
</style>
