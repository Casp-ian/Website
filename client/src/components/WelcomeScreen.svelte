<script>

    let screens = ["plain"];
    let screen = screens[Math.floor(Math.random()*screens.length)];
</script>


<div id="welcome">
    {#if screen = "plain"}
        <p>hello :)</p>
        <p>this panel is meant to contain a nice eye grabbing thing, but it isnt finished yet</p>
    {/if}
</div>


<style>
    #welcome {
        height: 90vh;
        justify-content: center;
        align-items: center;
        display: flex;
        flex-direction: column;

        background: var(--back-color);

        margin: .3rem;
        border-radius: .3rem;
    }
</style>
