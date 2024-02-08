<script>
    export let text = "No text...";
    export let focussed = false;

    function truncate(string, length) {
        return string.substring(0, length) + '...';
    }
</script>


<div id="blogItem">
    <div id="pointer">
        {"<"}
    </div>

    <div id="text">
        {#if focussed}
            <p>{text}</p>
        {:else}
            <p>{truncate(text, 4)}</p>
        {/if}
    </div>
</div>


<style>
    #blogItem {
        display: flex;
        align-items: center;
    }

    #text {
        border: black solid;
        border-radius: 10px;
    }
</style>