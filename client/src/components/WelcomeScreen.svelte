<script>
	import DvdBlock from '../components/dvdBlock.svelte';
	import WeatherMenu from '../components/WeatherMenu.svelte';
	import Chat from '../components/Chat.svelte';

	let screens = [
		DvdBlock,
		// WeatherMenu,  // TODO fix these / make these presentable
		// Chat, // TODO fix these / make these presentable
	];
	// if you change this every day it motivates users to check the page the next day, reserving a little space in their head rent free :)
	let screen = screens[new Date().getDay() % screens.length];
</script>

<!-- small little interesting thing on the homescreen -->
<div>
	<svelte:component this={screen} />
</div>

<style>
	div {
		height: 60vh;
		background-color: var(--accent-color);
		overflow: hidden;
	}
</style>
