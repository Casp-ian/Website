<script>

    let screens = ["plain"];
    let screen = screens[Math.floor(Math.random()*screens.length)];

    // change this to another image every day, or every reload.
    // if you change this every day it motivates users to check the page the next day, reserving a little space in their head rent free :)
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

        margin: .3rem;
        border-radius: .3rem;
    }
</style>
