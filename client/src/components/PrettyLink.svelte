<script>
    export let url;
    export let img = "https://t3.gstatic.com/faviconV2?client=SOCIAL&type=FAVICON&fallback_opts=TYPE,SIZE,URL&url=" + url + "&size=24";
</script>


<a href={url}>
    <img src={img} alt=""/>
    <p>
        <slot></slot>
    </p>
</a>


<style>
    a {
        display: flex;
        flex-direction: row;
        align-items: center;
        text-decoration: none;
        color: dimgray;
        gap: 2px
    }
</style>