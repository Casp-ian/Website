<script>
    import Cat from '$components/Cat.svelte';
    import Pane from "$components/Pane.svelte";
    import PrettyLink from "$components/PrettyLink.svelte";
</script>

<Cat startX={Math.random() * 500} startY={Math.random() * 500}/>

<Pane>
    <h1>haha funny kitty</h1>
    <p>meow meow
        <br/>
        you can pet it! :3</p>
    <p>
        This is not my original idea, oneko is a program that makes a cat chase your cursor. I think the original
        version of it is older than i am.</p>

    <p>
        But the first place i found it was
        <PrettyLink url="https://github.com/adryd325/oneko.js">this version.</PrettyLink>
        <br/>
        I made my own version without looking at other implementations tho, and added a simple petting mechanic.
    </p>
</Pane>


<style>
    pre, p, h1, h2, h3 {
        padding: 1rem;
        margin: 0;
    }
</style>
