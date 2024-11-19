<script lang="ts">
	import { goto } from '$app/navigation';
	import { onMount } from 'svelte';

	// TODO export as param?
	const fields = $state([
		{
			icon: '<',
			path: null,
			x: '150px',
			y: '50px',
		},
		{
			text: 'homeAAAAAAAAAAA',
			icon: '_',
			path: '/home',
			x: '300px',
			y: '50px',
		},
		{
			text: 'info',
			icon: '_',
			path: '/projects/this',
			x: '450px',
			y: '50px',
		},
		{
			text: 'kitty',
			icon: '_',
			path: '/projects/cat',
			x: '600px',
			y: '50px',
		},
		{
			icon: '>',
			path: null,
			x: '150px',
			y: '50px',
		},
	]);

	onMount(async () => {
		adjustButtons(); // to make sure the buttons start in the right spot
	})

	let distance = 0; // total number representing how far the nav is scrolled
	let touchDiff = 0; // movement between last 2 touch events

	let previousTouch: Touch;

	function scrollStart(event: TouchEvent) {
		// reset momentum
		clearInterval(momentumInterval);
		touchDiff = 0;

		previousTouch = event.changedTouches[0];
  }
	
	function scroll(event: TouchEvent) {
		touchDiff = (
			(previousTouch.screenX - event.changedTouches[0].screenX) +
			(event.changedTouches[0].screenY - previousTouch.screenY)
		) / 3;
		previousTouch = event.changedTouches[0];
		distance += touchDiff;

		adjustButtons();
	}
	
	function scrollEnd(_event: TouchEvent) {
		doMomentum(touchDiff);
	}

	const momentumMinimum = 3;
	const momentumFramerate = 25; //interval speed
	const momentumFalloff = 1.2;
	let momentumInterval: number | undefined;
	let momentumSpeed: number;
	function doMomentum(startSpeed: number) {
		momentumSpeed = startSpeed;
		
		momentumInterval = setInterval(() => {
			if (momentumSpeed <= momentumMinimum && momentumSpeed >= -momentumMinimum) {
				clearInterval(momentumInterval)
			}
		
			distance += momentumSpeed;
			momentumSpeed = momentumSpeed / momentumFalloff;

			adjustButtons();
		}, momentumFramerate)
	}

	const spaceBetween = 30; //px
	const radius = 300; //px
	const circumference = radius * Math.PI;
	const maxDistance = (circumference / 4) + (spaceBetween * (fields.length - 1));
	function adjustButtons() {
		// clamp distance
		if (distance <= 0) {
			distance = 0;
		}
		if (distance > maxDistance) {
			distance = maxDistance;
		}

		for (let i = 0; i < fields.length; i++) {
			let adjustedDistance = distance - (i * spaceBetween);
			let angleMovedFraction = adjustedDistance / circumference;
			let angle = angleMovedFraction * 2 * Math.PI;

			let distanceX = Math.sin(angle) * radius;
			let distanceY = Math.cos(angle) * radius;
			
			fields[i].x = distanceX + 'px';
			fields[i].y = distanceY + 'px';
		}
	}

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
		onclick={!!field.path ? () => goto(field.path) : null}
	>
		<pre>{field.icon}</pre>
		{#if !!field.text}
			<pre class="tag">{field.text}</pre>
		{/if}
	</div>
{/each}



<style>
	pre {
		margin: 0;
	}
	
	.item {
		z-index: 9;
		
		touch-action: none; /* disable scrolling when moving the nav */
		position: fixed;
		user-select: none;

		background-color: var(--interactable-color);
		height: 1rem;
		width: 1rem;

		padding: .5rem;
		border-radius: 100%;
		text-align: center;
	}

	.tag {
		position: relative;
		bottom: 1rem;
		right: 1rem;

		transform-origin: bottom right;
		transform: translate(-100%, -100%) rotate(45deg);
		
		background-color: var(--accent-color);
		width: fit-content;
	}
</style>
