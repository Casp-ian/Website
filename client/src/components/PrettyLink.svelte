<script>
    export let url;
    export let img;
</script>

<a href={url}>
    <img src={img} alt="" height="24"/>
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