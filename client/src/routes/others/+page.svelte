<script>
    import PrettyLink from "$components/PrettyLink.svelte";
</script>


<h1>Others</h1>
<p>I really like seeing other peoples personal websites, and following the links to the personal websites of their
    friends, or other sites they liked. It is pretty much what inspired me to make my own.</p>
<p>So i want to do the same, so here are some links to friends, cool sites and some of the sites that were an
    inspiration to me.</p>

<h2>Friends</h2>
<div class="list">
    <PrettyLink url="https://matpat.nl">Matpat :)</PrettyLink>
    <PrettyLink url="https://samme.xyz">Samme :3</PrettyLink>
</div>

<h2>Cool stuff</h2>
<div class="list">
    <PrettyLink url="https://shaneliesegang.com/projects/wasmbots/">Wasmbots</PrettyLink>
    <PrettyLink url="https://100r.co/site/about.html">100 rabbits</PrettyLink>

</div>

<h2>Buttons</h2>
<div class="grid">
    <a href="https://adryd.com/" target="_blank">
        <img src="https://adryd.com/static/buttons/adryd.png" alt="https://adryd.com/">
    </a>
    <a href="https://hl2.sh/" target="_blank">
        <img src="https://adryd.com/static/buttons/jay.png" alt="https://hl2.sh/">
    </a>
    <a href="https://astrid.sh/" target="_blank">
        <img src="https://astrid.sh/buttons/azzy.png" alt="https://astrid.sh/">
    </a>
</div>


<style>
    .list {
        display: flex;
        flex-direction: column;
    }

    .grid {
        image-rendering: pixelated;
        display: flex;
        flex-direction: row;
        gap: 1rem;
    }
</style>