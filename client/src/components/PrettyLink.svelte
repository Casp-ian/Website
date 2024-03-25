<script>
    export let url;
    export let img = "https://t3.gstatic.com/faviconV2?client=SOCIAL&type=FAVICON&fallback_opts=TYPE,SIZE,URL&url=" + url + "&size=24";
</script>


<a href={url} target="_blank">
    <img src={img} alt=""/>
    <b>
        <slot></slot>
    </b>
</a>


<style>
    a {
        display: flex;
        flex-direction: row;
        align-items: center;
        gap: 2px;
    }
</style>
