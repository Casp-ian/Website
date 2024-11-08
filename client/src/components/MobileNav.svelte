<script lang="ts">
	import { goto } from '$app/navigation';
	import { onMount } from 'svelte';

	// these 2 together is the offset used for the positions
	let distance = 0; // previously adjust movement
	let touchDiff = 0; // touch movement between touch start and end

	let firstTouch: TouchList;

	onMount(async () => {
		adjustButtons();
	})

	function scrollStart(event: TouchEvent) {
		// TODO disable scroll
		firstTouch = event.changedTouches;
	}
	
	function scroll(event: TouchEvent) {
		touchDiff = (
			(firstTouch[0].screenX - event.changedTouches[0].screenX) +
			(event.changedTouches[0].screenY - firstTouch[0].screenY)
		) / 3;

		adjustButtons();
	}
	
	function scrollEnd(event: TouchEvent) {
		distance = distance + touchDiff;
		touchDiff = 0;

		// TODO reenable scroll
		// TODO momentum
	}

	function adjustButtons() {
		let movement = distance + touchDiff;
		for (let i = 0; i < fields.length; i++) {
			fields[i].x = movement + i * 50 + 'px';
			fields[i].y = movement + i * 50 + 'px';
		}
	}

	const fields = $state([
		{
			text: '<',
			path: null,
			x: '150px',
			y: '50px',
		},
		{
			text: 'test',
			path: '/home',
			x: '300px',
			y: '50px',
		},
		{
			text: 'wasd',
			path: '/projects',
			x: '450px',
			y: '50px',
		},
		{
			text: 'dog',
			path: '/home',
			x: '600px',
			y: '50px',
		},
	]);
</script>

{#each fields as field}
	<!-- TODO accessibility is imnportant!!! -->
	<!-- svelte-ignore a11y_click_events_have_key_events -->
	<!-- svelte-ignore a11y_no_static_element_interactions -->
	<div
		class="item"
		ontouchstart={scrollStart}
		ontouchmove={scroll}
		ontouchend={scrollEnd}
		style="bottom: {field.y}; right: {field.x}"
		onclick={field.path !== null ? () => goto(field.path) : null}
	>
		<pre>{field.text}</pre>
	</div>
{/each}

<style>
	.item {
		background-color: var(--interactable-color);
		z-index: 9;
		position: fixed;
		user-select: none;
		padding-left: 0.5rem;
		padding-right: 0.5rem;
	}
</style>
