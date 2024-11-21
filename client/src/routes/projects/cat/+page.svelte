<script>
	import Cat from '$components/Cat.svelte';
</script>

{#each {length: 3} as _, i}
	<Cat startX={Math.random() * 500} startY={Math.random() * 500} />
{/each}

<h1>haha funny kitty</h1>
<p>meow meow</p>
<p>you can pet it :3</p>
