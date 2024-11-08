<script>
	import PrettyLink from './PrettyLink.svelte';
</script>

<div id="links">
	<hr />
	<PrettyLink url="github.com/Casp-ian">github</PrettyLink>
	<PrettyLink url="github.com/Casp-ian">matpat :)</PrettyLink>
	<PrettyLink url="samme.xyz">samme :3</PrettyLink>
	<PrettyLink url="www.youtube.com/watch?v=dQw4w9WgXcQ">
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
