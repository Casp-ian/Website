<script>
    import Cat from '$components/Cat.svelte';
    import Pane from "$components/Pane.svelte";
</script>

    <Cat startX={Math.random() * 500} startY={Math.random() * 500}/>

<Pane>
    <h1>haha funny kitty</h1>
    <p>meow meow</p>
    <p>you can pet it :3</p>
</Pane>


<style>
    pre, p, h1, h2, h3 {
        padding: 1rem;
        margin: 0;
    }
</style>