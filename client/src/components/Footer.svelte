<script>
	import PrettyLink from './PrettyLink.svelte';
</script>

<div id="links">
	<hr />
	<PrettyLink url="https://github.com/Casp-ian">github</PrettyLink>
	<PrettyLink url="https://matpat.nl">matpat :)</PrettyLink>
	<PrettyLink url="https://samme.xyz">samme :3</PrettyLink>
	<PrettyLink url="https://www.youtube.com/watch?v=dQw4w9WgXcQ">
		i have no other links :)
	</PrettyLink>
</div>

<style>
	hr {
		color: var(--back-color2);
		width: 100%;
	}

	#links {
		overflow: clip;

		padding: 1rem;
		display: flex;
		flex-direction: column;
		justify-content: space-evenly;
	}
</style>
