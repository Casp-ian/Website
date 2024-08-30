<script>
    export let params;
    // should contain errorCode and message
</script>

<div id="wrapper">
    <h1>Something went wrong</h1>
    {#if params.message}
        <p>{params.message}</p>
    {/if}
    
    <!--there might be a neater way to write this, but i dont know how :P-->
    {#if !params.message}
        <p>we dont know what went wrong :(</p>
    {/if}

    <a href="/home">go back</a>
</div>


<style>
    #wrapper {
        margin: 2rem 5rem 2rem 5rem;
    }
</style>
