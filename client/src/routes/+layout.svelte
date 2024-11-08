<script>
	import { isTouch, isVertical } from '$lib/Responsiveness';
	import Sidebar from '$components/Sidebar.svelte';
	import MobileNav from '$components/MobileNav.svelte';
	import Notifications from '$components/Notifications.svelte';
	import Weather from '$components/Weather.svelte';
	import { onMount } from 'svelte';
	import WelcomeScreen from '$components/WelcomeScreen.svelte';
	import VerticalNav from '$components/VerticalNav.svelte';

	let vertical = $state(false);
	let touch = $state(false);

	onMount(() => {
		vertical = isVertical();
		touch = isTouch();

		addEventListener('resize', () => (vertical = isVertical()));
	});

	let { children } = $props();
</script>

<main>
	{#if vertical}
		{#if touch}
			<MobileNav />
			<!-- todo header for touch screens -->
		{:else}
			<VerticalNav />
			<!-- todo header for vertical screens -->
		{/if}

		<Notifications />
		<Weather />
		<div id="mobileWrapper">
			{@render children()}
		</div>
	{:else}
		<div id="sidebar">
			<Sidebar />
		</div>

		<div id="page">
			{@render children()}
		</div>

		<div id="extra">
			<Notifications />
			<Weather />
		</div>
	{/if}
</main>

<style>
	:global(*) {
		--main-color: #f7fff7;
		--back-color: #240021;
		--back-color2: #3d003a;
		--interactable-color: #ed455c;
		--accent-color: #8338ec;
		transition: background-color 0.2s;
	}

	:global(p, h1, h2, h3, button, pre) {
		color: var(--main-color);
		overflow: hidden;
		overflow-wrap: break-word;
	}

	:global(a) {
		color: var(--interactable-color);
	}

	:root {
		font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, Cantarell,
			'Open Sans', 'Helvetica Neue', sans-serif;
	}

	main {
		display: flex;
		flex-direction: row;
		flex-wrap: nowrap;

		position: absolute;
		top: 0;
		left: 0;
		min-height: 100vh;
		width: 100vw;
		background-color: var(--back-color);

		/* TODO does this even do anything? it is supposed to stop shit from adding scroll if they go off screen */
		/* clip: rect(0, auto, auto, 0); */
	}

	#sidebar {
		width: 18vw;
		padding: 1vw;
	}

	#page {
		box-shadow: 0 0 0.5rem var(--back-color2);
		width: 50vw;
		padding: 2rem 5rem 2rem 5rem;
		overflow: hidden;
	}

	#extra {
		width: 18vw;
		padding: 1vw;
	}

	#mobileWrapper {
		width: 100%;
		padding: 2rem 5rem 2rem 5rem;
		overflow: hidden;
	}
</style>
