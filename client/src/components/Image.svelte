<script>
    export let image;
    export let placeholderImage = "/src/assets/itemPlaceholder.png";

    export let width = '10rem';
    export let height = '10rem';
</script>

<div id="image" style="--width: {width}; --height: {height}">
    {#if image}
        <img src={image} alt="item image">
    {:else}
        <img src={placeholderImage} alt="placeholder image">
    {/if}
</div>


<style>
    img {
        width: var(--width);
        height: var(--height);
    }
</style>