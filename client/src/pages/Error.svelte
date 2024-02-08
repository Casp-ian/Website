<script>
    export let params;
</script>


<h1>Something went wrong</h1>
{#if params.errorCode}
    <p>error code: {params.errorCode}</p>
{/if}
{#if params.message}
    <p>{params.message}</p>
{/if}

<!--there might be a neater way to write this, but i dont know how :P-->
{#if !params.message && !params.errorCode}
    <p>we dont know what went wrong :(</p>
{/if}
