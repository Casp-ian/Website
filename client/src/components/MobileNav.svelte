<script lang="ts">
	import { goto } from '$app/navigation';

	let distance = 0;
	let lastTouch: TouchList;

	// it just passes all move events when you press it and drag away, and stop when you release
	function scroll(event: TouchEvent) {
		// TODO disable scrolling when this is done
		if (lastTouch === undefined) {
			lastTouch = event.changedTouches;
			return;
		}
		let touchDiff =
			(event.changedTouches[0].screenX -
				lastTouch[0].screenX +
				(event.changedTouches[0].screenY - lastTouch[0].screenY)) /
			5;
		distance += touchDiff;
		console.log('move', event);
		adjustButtons();
	}

	function adjustButtons() {
		for (let i = 0; i < fields.length; i++) {
			fields[i].x = distance + i * 50 + 'px';
			fields[i].y = distance + i * 50 + 'px';
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

<div id="wrapper">
	<div id="bar">
		{#each fields as field}
			<!-- TODO accecibility is imnportant!!! -->
			<!-- svelte-ignore a11y_click_events_have_key_events -->
			<!-- svelte-ignore a11y_no_static_element_interactions -->
			<div
				class="item"
				ontouchmove={scroll}
				style="bottom: {field.y}; right: {field.x}"
				onclick={field.path !== null ? () => goto(field.path) : null}
			>
				<!-- todo debate to use an a or button element or not, it does cause issues on firefox and safari mobile at least -->
				<!-- but a and button elements do have better default accessibility -->
				<p>{field.text}</p>
			</div>
		{/each}
	</div>
</div>

<style>
	#wrapper {
		/* position to be right side, one third from the bottom */
		/* todo lefthanded version */
		position: absolute;
		right: 0;
		bottom: 33vh;
	}

	.item {
		position: fixed;
		user-select: none;
		padding-left: 0.5rem;
		padding-right: 0.5rem;
	}
</style>
