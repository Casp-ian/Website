<script>
    export let url;
    export let img = "https://t3.gstatic.com/faviconV2?client=SOCIAL&type=FAVICON&fallback_opts=TYPE,SIZE,URL&url=" + url + "&size=24";
</script>


<a href={url} target="_blank">
    {#if (img !== null)}
        <img src={img} width={24} alt=""/>
    {/if}
    <b>
        <slot></slot>
    </b>
</a>


<style>
    a {
        display: inline-flex;
        flex-direction: row;
        align-items: center;
        gap: 2px;
        width: max-content;
    }
</style>
