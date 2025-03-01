<script>
    export let url;

    import link from '$assets/icons/link.svg';
    export let img = link;

    // NOTE: it is cool to get the favicon of the linked websites, but relying on this random api is kinda eh
    // export let img =
    //     'https://t3.gstatic.com/faviconV2?client=SOCIAL&type=FAVICON&fallback_opts=TYPE,SIZE,URL&url=' +
    //     url +
    //     '&size=24';
</script>

<a href={url} target="_blank">
    {#if img !== null}
        <img src={img} width={19} alt=""/>
    {/if}
    <b>
        <slot></slot>
    </b>
</a>

<style>
    /* NOTE this will probably break at other font sizes*/
    a {
        margin-top: -2px;
        overflow: clip;
        display: inline-flex;
        flex-direction: row;
        vertical-align: middle;
    }
    img {
        vertical-align: middle;
    }
</style>
